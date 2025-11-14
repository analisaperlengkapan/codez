# Codeza

Codeza adalah monorepo platform backend yang sedang dikembangkan dengan Rust. Proyek ini fokus pada integrasi Git (Gitea/GitLab), orkestrasi CI/CD sederhana, dan API Gateway yang menyatukan beberapa layanan.

README ini mendeskripsikan **kondisi aktual** repo per hari ini berdasarkan struktur dan kode yang ada, bukan visi jangka panjang.

---

## Arsitektur Tingkat Tinggi

Repo ini adalah workspace Cargo dengan beberapa crate utama:

- `crates/shared`
- `crates/api-gateway`
- `crates/auth-service`
- `crates/git-service`
- `crates/cicd-engine`
- `crates/registry`
- `crates/mfe-manager`
- `crates/msr`
- `crates/orchestrator`

Tidak ada UI/frontend di repo ini; fokusnya adalah layanan backend berbasis HTTP dan integrasi layanan lain (PostgreSQL, Redis, MinIO).

### 1. `crates/shared`

Berisi utilitas lintas layanan:

- `Config` – konfigurasi terpusat (server, database, redis, JWT, GitConfig, dll).
- `CodezaError` – enum error umum yang dipakai service lain.
- `logging` – inisialisasi `tracing`/logging.
- `middleware` – middleware Axum (request ID, logging, dsb.).
- `auth` – helper autentikasi (password hashing dengan Argon2, JWT, dll.).
- `metrics` – registry metrik in-memory (`MetricsRegistry`, `Counter`, `Gauge`).
- `tracing_module` – struktur dasar untuk tracing terdistribusi.
- Modul analitik/dashboard/report yang saat ini sebagian besar masih model & stub.

### 2. `crates/api-gateway`

API Gateway utama yang mengekspos endpoint HTTP ke dunia luar. Menggunakan Axum.

Fitur utama yang **sudah ada**:

- Health check: `GET /health`
- Info root: `GET /`
- Auth:
  - `POST /auth/register`
  - `POST /auth/login`
  - `GET /auth/user`
- Git repository CRUD (melalui `git-service`):
  - `POST /api/v1/repos`
  - `GET /api/v1/repos/:owner`
  - `GET /api/v1/repos/:owner/:repo`
  - `DELETE /api/v1/repos/:owner/:repo`
- Webhook Git (Gitea / GitLab):
  - `POST /api/v1/git/webhook`
  - Validasi signature/token sesuai provider.
  - Parse push event dan memicu pipeline di `cicd-engine`.
- CI/CD read API:
  - `GET /api/v1/pipelines` – list eksekusi pipeline yang tersimpan di PostgreSQL.
  - `GET /api/v1/pipelines/:id` – detail eksekusi pipeline.
  - `GET /api/v1/pipelines/:id/jobs` – daftar eksekusi job untuk pipeline tersebut.
- Observability:
  - `GET /metrics` – mengembalikan `MetricValue[]` dari `MetricsRegistry` sebagai JSON.
  - Metrik otomatis untuk hit webhook dan trigger pipeline.
  - Span tracing sederhana pada jalur webhook dan CI trigger.

Struktur internal sudah dipecah per modul routing:

- `routing.rs` – definisi `AppState` (PgPool + Config + MetricsRegistry) dan `build_routes()`.
- `routing/auth.rs` – handler auth.
- `routing/git.rs` – handler repo CRUD + helper `build_git_provider_config`.
- `routing/cicd.rs` – handler list/detail pipeline & job execution.
- `routing/webhook.rs` – handler webhook Gitea/GitLab + integrasi CI/CD.

### 3. `crates/auth-service`

Layanan auth terpisah yang diakses Gateway lewat fungsi handler:

- Registrasi user.
- Login dan JWT.
- Endpoint `get_current_user` untuk `GET /auth/user` di Gateway.

Detail storage (tabel user, dsb.) berada di layer SQLX dan mengikuti konfigurasi `Config.database`.

### 4. `crates/git-service`

Abstraksi multi-provider untuk operasi Git dasar dan webhook payload:

- `ProviderType` – `Gitea`, `GitLab`, `GitHub` (GitHub belum diimplementasi).
- `GitProvider` trait – operasi:
  - Repo: create/get/list/delete.
  - User & organization: create/get.
  - **Baru**: `get_file_contents(owner, repo, path, ref)` untuk membaca raw file dari repo.
- Provider konkret:
  - `GiteaProvider` – memakai REST API Gitea.
  - `GitLabProvider` – memakai REST API GitLab v4.
- `RepositoryService` – service level di atas trait provider (dipakai API Gateway).
- `webhook` – definisi payload push/pull_request/issue dan `WebhookValidator` untuk HMAC.

Saat ini hanya subset API Git yang diimplementasikan (cukup untuk demo repo CRUD dan webhook CI).

### 5. `crates/cicd-engine`

Mesin CI/CD ringan yang dipakai webhook untuk memicu pipeline:

- Model pipeline:
  - `Pipeline`, `Stage`, `Job`.
  - `PipelineExecution`, `StageExecution`, `JobExecution`, `PipelineStatus`.
- Executor:
  - `JobExecutor` trait.
  - `LocalJobExecutor` – menjalankan job secara lokal (simulasi / stub eksekusi).
  - `DockerJobExecutor` – didefinisikan tapi belum benar-benar di-wire untuk runtime.
- Trigger helper:
  - `GitPushContext` – konteks normalisasi push (provider, repo, ref, commit).
  - `trigger_push_pipeline` – membuat pipeline stub sederhana (1 stage, 1 job) dan mengeksekusi 1 job.
  - `pipeline_from_yaml_str` – membangun `Pipeline` dari YAML sederhana.
  - `trigger_push_pipeline_from_yaml` – mencoba eksekusi pipeline yang didefinisikan di YAML, dengan fallback ke stub default jika YAML invalid atau tidak punya job.

Integrasi dengan API Gateway:

- Webhook Gitea/GitLab membangun `GitPushContext` dari payload.
- Gateway memanggil `trigger_push_pipeline_from_yaml` **jika** menemukan file `codeza-ci.yml` di repo (via `git-service`).
- Jika file YAML tidak ada/invalid → fallback ke `trigger_push_pipeline`.
- Hasil pipeline dan (opsional) job execution dipersist ke tabel `ci_pipeline_executions` dan `ci_job_executions` di PostgreSQL.

### 6. Crate lain

Crate berikut sudah ada namun tidak semua fungsinya dipakai penuh di alur Git/CI saat ini:

- `registry` – kemungkinan untuk image/artifact registry; masih minim integrasi.
- `mfe-manager` – modul terkait micro frontends (belum tersambung dengan gateway di level API).
- `msr` – modul terkait source code/mining repository (masih awal).
- `orchestrator` – orkestrasi antar layanan; masih tahap awal.

README ini hanya menyorot bagian yang aktif dipakai di jalur Git → Webhook → CI/CD → DB → API.

---

## Alur Git → CI/CD Secara Singkat

1. **Push ke Gitea/GitLab** pada repo yang terhubung ke Codeza.
2. Provider mengirim webhook ke `POST /api/v1/git/webhook`.
3. API Gateway:
   - Memvalidasi signature/token webhook.
   - Parse event push.
   - Bangun `GitPushContext`.
   - Coba baca `codeza-ci.yml` lewat `git-service`:
     - Jika ada dan valid → `trigger_push_pipeline_from_yaml`.
     - Kalau tidak → `trigger_push_pipeline` (stub pipeline default).
4. `cicd-engine` mengeksekusi 1 job (via `LocalJobExecutor`) dan mengembalikan `TriggerResult`.
5. API Gateway menyimpan info eksekusi ke PostgreSQL:
   - `ci_pipeline_executions`.
   - `ci_job_executions`.
6. Status dapat diambil lewat:
   - `GET /api/v1/pipelines`.
   - `GET /api/v1/pipelines/:id`.
   - `GET /api/v1/pipelines/:id/jobs`.

---

## Menjalankan Secara Lokal

### 1. Prasyarat

- Rust toolchain yang mendukung edition 2024 (lihat `[workspace.package]` di `Cargo.toml`).
- Docker (untuk menjalankan Postgres, Redis, MinIO).

### 2. Menjalankan dependency lewat Docker

Di root repo `codeza/`:

```bash
docker-compose up -d
```

Ini akan menjalankan:

- PostgreSQL `codeza_dev` (user/password `codeza`).
- Redis.
- MinIO (akses via `http://localhost:9000` / `:9001` dengan kredensial default).

### 3. Konfigurasi environment

Salin `.env.example` bila diperlukan dan sesuaikan dengan kebutuhan lokal (database URL, secret JWT, dsb.). 
Saat ini banyak komponen memakai `Config::default_dev()` sehingga bisa jalan dengan nilai default selama Postgres dan Redis sesuai `docker-compose.yml`.

### 4. Menjalankan API Gateway

Di root repo:

```bash
cargo run -p codeza-api-gateway
```

API akan listen di host/port sesuai `Config.server` (default dev menggunakan nilai yang didefinisikan di `Config::default_dev()`).

### 5. Testing

Beberapa test unit sudah ada (misalnya di `shared`, `cicd-engine`, `api-gateway`).

Contoh menjalankan test partial:

```bash
cargo test -p codeza-api-gateway
```

Saat ini, ada kemungkinan build/test gagal di crate eksternal `jsonwebtoken` (versi `10.2.0`) tergantung versi Rust toolchain yang digunakan. Masalah ini berada di luar kode Codeza dan mungkin membutuhkan penyesuaian versi Rust atau dependency di lingkungan lokal.

---

## Status & Keterbatasan Saat Ini

Hal-hal berikut **belum** selesai atau masih minimal:

- **GitHub provider** belum diimplementasikan (hanya Gitea & GitLab yang aktif).
- **Eksekusi job** masih memakai `LocalJobExecutor` (belum ada orkestrasi runner/dokumenasi Docker runner yang matang).
- **YAML pipeline**:
  - Formatnya sengaja sederhana.
  - Hanya job pertama pada stage pertama yang dieksekusi pada jalur webhook.
- **Observability**:
  - Metrik dan tracing internal sudah ada, tetapi belum diekspos dalam format Prometheus atau OpenTelemetry penuh.
- **Error handling**:
  - Sebagian besar jalur webhook/CI menggunakan pendekatan "best-effort": kegagalan persist ke DB atau kegagalan eksekusi job dicatat di log tetapi tidak menggagalkan respon webhook (supaya provider tidak terus retry).
- **Dokumentasi API** masih terbatas pada README ini dan komentar di kode; belum ada OpenAPI/Swagger resmi.

---

## File Dokumentasi Pendukung

Di root repo tersedia beberapa dokumen progres/roadmap:

- `COMPARISON_AND_ROADMAP.md` – perbandingan dan rencana evolusi Codeza.
- `PHASE_X_*.md` – catatan perkembangan bertahap (phase 1, 2, dst.).

Dokumen-dokumen ini menggambarkan arah jangka panjang, sementara README ini fokus pada apa yang sudah benar-benar ada di kode.

---

## Kontribusi

Proyek ini masih dalam tahap aktif refactor dan eksperimen. Belum ada panduan kontribusi formal (CONTRIBUTING.md), namun pola umum yang dipakai:

- Mengutamakan modularitas per crate.
- Menggunakan `codeza_shared::Config` dan `AppState` untuk menghindari akses langsung `std::env` di handler.
- Menjaga jalur Git → Webhook → CI/CD tetap sederhana dan mudah diobservasi.

Jika ingin menambah fitur, disarankan untuk mengikuti pola yang sudah ada di `git-service`, `cicd-engine`, dan `api-gateway`.

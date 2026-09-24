# Codeza

[![CI](https://github.com/analisaperlengkapan/codez/actions/workflows/ci.yml/badge.svg)](https://github.com/analisaperlengkapan/codez/actions/workflows/ci.yml)

Codeza is a lightweight, Gitea-inspired code hosting platform written in Rust. It pairs an
**Axum** HTTP backend with a **Leptos** (Rust/WASM) single-page frontend, sharing one set of
typed DTOs across both.

> **Note:** persistence is currently in-memory so the demo runs with zero setup. Data resets
> when the backend restarts. A PostgreSQL backend is wired up and enabled by setting
> `DATABASE_URL`.

## Tech stack

| Layer | Technology |
| --- | --- |
| Backend | [Axum](https://github.com/tokio-rs/axum), Tokio, Tower |
| Frontend | [Leptos](https://github.com/leptos-rs/leptos) (Rust/WASM), Trunk |
| Shared | Common DTOs and utilities in `crates/shared` |
| Storage | In-memory by default; PostgreSQL via `tokio-postgres` |
| Build | Cargo workspaces + Trunk |
| Tests | `cargo test` (backend) + Playwright (end-to-end) |
| CI | GitHub Actions — fmt, clippy, tests, frontend build, e2e |

## Architecture

The backend is split into one module per domain under `crates/backend/src/handlers/`, with
per-area sub-routers assembled in `crates/backend/src/routes/`. The large repository surface
lives in `handlers/repo/` (one file per area: repos, issues, pulls, comments, contents,
webhooks, wiki, pulse). All shared state lives behind `RwLock`s in `state.rs` that are
poison-safe (`.unwrap_or_else(|e| e.into_inner())`), so a panicking handler never takes the
whole server down. DTOs live in `crates/shared`, one module per domain, re-exported from its
root so both crates keep a flat `shared::<Type>` import path.

The frontend is a Leptos SPA with one page module per feature area (and a `repo/` directory
module for the larger repository surfaces), all sharing a single `style.css` design system
built on tokens. Every request goes through the typed client in `crates/frontend/src/api.rs`,
which centralises URL building and error handling so pages never hand-roll fetch chains.

Repository pages share a common header and section navigation via the `RepoNav` component,
which highlights the active section and appears on every repository sub-page.

## Screenshots

Every screen below is captured from a running instance by
[`scripts/capture_screenshots.py`](scripts/capture_screenshots.py).

### Discovery & dashboard

| Dashboard | Explore | Repository |
| --- | --- | --- |
| ![Dashboard](docs/screenshots/01_dashboard.png) | ![Explore](docs/screenshots/02_explore.png) | ![Repository](docs/screenshots/03_repo_detail.png) |

### Repository code

| Code browser | Commits | Branches |
| --- | --- | --- |
| ![Code](docs/screenshots/04_repo_code.png) | ![Commits](docs/screenshots/05_commits.png) | ![Branches](docs/screenshots/06_branches.png) |

| Tags | Releases | Pulse |
| --- | --- | --- |
| ![Tags](docs/screenshots/07_tags.png) | ![Releases](docs/screenshots/14_releases.png) | ![Pulse](docs/screenshots/17_pulse.png) |

### Issues & pull requests

| Issues | Issue detail | Pull requests |
| --- | --- | --- |
| ![Issues](docs/screenshots/08_issues.png) | ![Issue detail](docs/screenshots/09_issue_detail.png) | ![Pulls](docs/screenshots/10_pulls.png) |

| Pull request detail | Labels | Milestones |
| --- | --- | --- |
| ![Pull detail](docs/screenshots/11_pull_detail.png) | ![Labels](docs/screenshots/12_labels.png) | ![Milestones](docs/screenshots/13_milestones.png) |

### Collaboration

| Discussions | Projects | Wiki |
| --- | --- | --- |
| ![Discussions](docs/screenshots/18_discussions.png) | ![Projects](docs/screenshots/19_projects.png) | ![Wiki](docs/screenshots/20_wiki.png) |

| Wiki editor | Collaborators | Packages |
| --- | --- | --- |
| ![Wiki editor](docs/screenshots/36_wiki_edit.png) | ![Collaborators](docs/screenshots/21_collaborators.png) | ![Packages](docs/screenshots/23_packages.png) |

### Automation, security & comparison

| Actions | Workflow run | Security |
| --- | --- | --- |
| ![Actions](docs/screenshots/15_actions.png) | ![Workflow run](docs/screenshots/39_workflow_runs.png) | ![Security](docs/screenshots/16_security.png) |

| Compare | Code search | Notifications |
| --- | --- | --- |
| ![Compare](docs/screenshots/34_compare.png) | ![Code search](docs/screenshots/35_code_search.png) | ![Notifications](docs/screenshots/24_notifications.png) |

### Account & administration

| Search | Admin | Admin users |
| --- | --- | --- |
| ![Search](docs/screenshots/25_search.png) | ![Admin](docs/screenshots/26_admin.png) | ![Admin users](docs/screenshots/27_admin_users.png) |

| User profile | Followers | Organization |
| --- | --- | --- |
| ![User profile](docs/screenshots/28_user_profile.png) | ![Followers](docs/screenshots/47_user_followers.png) | ![Organization](docs/screenshots/29_org_profile.png) |

| Settings | Login | Register |
| --- | --- | --- |
| ![Settings](docs/screenshots/30_settings.png) | ![Login](docs/screenshots/31_login.png) | ![Register](docs/screenshots/32_register.png) |

### Repository management

| Create repository | Migrate repository | Create organization |
| --- | --- | --- |
| ![Create repo](docs/screenshots/33_create_repo.png) | ![Migrate repo](docs/screenshots/46_migrate_repo.png) | ![Create org](docs/screenshots/45_org_create.png) |

| Repository settings | Branch protection | Webhooks |
| --- | --- | --- |
| ![Repo settings](docs/screenshots/22_repo_settings.png) | ![Branch protection](docs/screenshots/40_settings_branches.png) | ![Webhooks](docs/screenshots/41_settings_webhooks.png) |

| Secrets | Deploy keys | Git LFS |
| --- | --- | --- |
| ![Secrets](docs/screenshots/42_settings_secrets.png) | ![Deploy keys](docs/screenshots/43_settings_keys.png) | ![Git LFS](docs/screenshots/44_settings_lfs.png) |

| Milestone detail | Release detail |
| --- | --- |
| ![Milestone detail](docs/screenshots/37_milestone_detail.png) | ![Release detail](docs/screenshots/38_release_detail.png) |

## Features

- **Repositories** — create, list, fork, transfer, and configure, with a code browser,
  commit history, branches, and tags.
- **Issues & pull requests** — full lifecycle with comments, labels, milestones, assignees,
  reviews, and pagination/sorting.
- **Discussions** — community discussions with categories and comments.
- **Releases & packages** — releases with assets plus a package registry.
- **Organizations & teams** — organizations, teams, memberships, and audit logs.
- **Users** — authentication, profile settings, SSH/GPG keys, and 2FA settings.
- **Social** — star and watch repositories, follow users, and activity feeds.
- **Developer settings** — OAuth2 applications and personal access tokens.
- **Wiki** — integrated per-repository documentation.
- **Git LFS** — basic large-file-storage lock management.
- **Webhooks** — repository webhooks with delivery history and SSRF protection.
- **Secrets & keys** — repository secrets and deploy keys.
- **Actions** — workflow runs with logs and re-run controls.
- **Security** — vulnerability and secret-scanning dashboards.

## Getting started

### Prerequisites

- Rust (latest stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `trunk`: `cargo install trunk`

### Build and run

```bash
# 1. Build the frontend into crates/frontend/dist
(cd crates/frontend && trunk build)

# 2. Run the backend, which also serves the built frontend
cargo run -p backend
```

Open <http://127.0.0.1:3000>. The backend serves both the API (`/api/v1/*`) and the SPA, so
client-side deep links work without a separate dev server.

For frontend hot-reload during development, run `trunk serve` in `crates/frontend` instead
and point it at the backend API.

### Configuration

All settings are environment variables with sensible defaults:

| Variable | Default | Purpose |
| --- | --- | --- |
| `CODEZA_HOST` | `127.0.0.1` | Bind address |
| `CODEZA_PORT` | `3000` | Bind port |
| `CODEZA_STATIC_DIR` | `crates/frontend/dist` | Compiled frontend assets |
| `DATABASE_URL` | *(unset)* | Enables PostgreSQL persistence |

## Development

```bash
cargo fmt --all                        # format
cargo clippy --workspace --all-targets # lint
cargo test --workspace                 # run all tests
```

### Layout

```
crates/
  backend/    Axum API server — routes/ (sub-routers), handlers/ (one per
              domain), state.rs, seed.rs, tests/
  frontend/   Leptos SPA (pages/, components/) + api.rs client + style.css
  shared/     DTOs and helpers, one module per domain, re-exported from lib.rs
docs/
  screenshots/  UI reference images used in this README
scripts/        Verification and screenshot tooling
tests/          Playwright end-to-end specs
```

### Responsive design

The layout is audited at 390 px, 768 px, and 1440 px. The global header scrolls
horizontally on narrow screens instead of wrapping, so the document never overflows.
`scripts/capture_screenshots.py` drives the screenshot gallery; `python scripts/audit_responsive.py`
sweeps every route at three widths and reports any element that escapes the viewport.

### Verification & screenshots

With a server running (default `http://127.0.0.1:3000`; override with `CODEZA_BASE_URL`):

```bash
python scripts/capture_screenshots.py docs/screenshots   # refresh README images
python scripts/verify_reviews.py                         # exercise a feature flow
```

The `tests/` directory holds Playwright specs. They target `http://127.0.0.1:8080`
(the `trunk serve` origin, which proxies `/api/v1` to the backend) and honour the
`CODEZA_BASE_URL` override:

```bash
(cd crates/frontend && trunk serve)   # UI + API proxy on :8080
npx playwright test --project=chromium
```

## API

API endpoints are prefixed with `/api/v1` and follow GitHub-style REST conventions.

## Contributing

Contributions are welcome — see [`CONTRIBUTING.md`](CONTRIBUTING.md) for setup, the
conventions we follow, and the checks CI runs. Security issues should be reported
privately per [`SECURITY.md`](SECURITY.md), not in a public issue.

## License

Released under the [MIT License](LICENSE).


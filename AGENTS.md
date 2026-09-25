# AGENTS.md

Guidance for AI agents and contributors working in this repository.

## Project overview

Codeza is a Gitea-like code hosting platform: an **Axum** backend and a **Leptos**
(Rust/WASM) SPA, with shared DTOs in a third crate. Storage is in-memory by default and
optionally PostgreSQL when `DATABASE_URL` is set.

## Repository layout

```
crates/backend     Axum API server
  src/
    main.rs        Entry point + SPA/static fallback
    config.rs      Env-var configuration
    routes/        HTTP route registration, one sub-router per area
                   (repos, issues, pulls, orgs, users, admin, actions,
                   packages, projects, releases, misc) + mod.rs
    state.rs       Shared AppState (in-memory stores)
    handlers/      Feature handlers, one module per domain
      repo/        Repository handlers split by area (repos, issues, pulls,
                   comments, contents, webhooks, wiki, pulse) + shared helpers
      org/         Organization handlers (admin.rs, meta.rs) + shared helpers
    db.rs          Optional PostgreSQL layer
    seed.rs        Demo in-memory state
    tests/         Integration tests, one module per area + mod.rs
crates/frontend    Leptos SPA
  src/
    main.rs        Router definitions
    api.rs         Typed HTTP client helpers (fetch + write wrappers)
    components/    Reusable UI (nav.rs, repo_nav.rs)
    pages/         One module per feature area
  style.css        Design system (tokens + components + utilities)
  index.html       Trunk entry (pulls in style.css)
crates/shared      DTOs and helpers shared by both crates
  src/             One module per domain (repos, issues, pulls, users, …)
                   re-exported from lib.rs so `shared::<Type>` keeps working
docs/screenshots   README gallery images
scripts/           Verification + screenshot tooling
tests/             Playwright end-to-end specs
```

## Build, test, lint

```bash
# Frontend must be built before the backend can serve it
(cd crates/frontend && trunk build)

cargo run -p backend                    # serve API + SPA on :3000
cargo test --workspace                  # all tests (backend + shared)
cargo clippy --workspace --all-targets  # lint (keep this clean)
cargo fmt --all                         # format
```

Test suites:

- **Backend integration** (`crates/backend/src/tests/`, one module per area) drive the real
  router via `tower::ServiceExt::oneshot` against the seeded in-memory state.
- **Backend unit** tests are colocated with the pure helpers they cover (e.g.
  `handlers/repo/mod.rs` tests `process_mentions`, `process_closers`, `is_private_ipv4`).
- **Shared** DTO/serde tests live in `crates/shared`.
- **Frontend e2e** (`tests/`) is Playwright: feature specs plus `routes.spec.ts`, which
  smoke-renders every client route and asserts no uncaught page errors.

CI (`.github/workflows/ci.yml`) runs the same checks on every push/PR: `cargo fmt
--check`, `cargo clippy -- -D warnings`, `cargo test`, a release `trunk build`, and the
Playwright e2e suite against a locally started backend + `trunk serve`. The `rust`,
`frontend`, and `e2e` jobs are independent (each in its own checkout), so the e2e job
builds the bundle it needs via `trunk serve` rather than consuming another job's output.

The backend serves the SPA from `CODEZA_STATIC_DIR` (default `crates/frontend/dist`). Deep
links return `index.html`; unknown `/api/*` paths return `404`.

## Conventions

### Rust

- Keep `cargo clippy --workspace --all-targets` warning-free.
- One handler module per domain under `crates/backend/src/handlers/`. Large domains use a
  directory module (e.g. `handlers/repo/`, `handlers/org/`) with a `mod.rs` that holds
  shared imports and helpers, and re-exports each submodule so callers keep using
  `handlers::<name>`.
- Routing lives in `crates/backend/src/routes/`, one sub-router per area, assembled in
  `routes/mod.rs`. Handlers never register routes themselves.
- Shared DTOs live in `crates/shared/src/`, one module per domain, all re-exported from
  `lib.rs` so downstream code keeps the flat `shared::<Type>` path. Do not glob-import
  `shared::*` in the backend; the re-exports are the public surface.
- Frontend pages: one file per feature area under `crates/frontend/src/pages/`; large
  features (e.g. `repo`) use a directory module with a re-exporting `mod.rs`.
- All frontend HTTP goes through `crates/frontend/src/api.rs`. Pages must not call
  `gloo_net::http::Request` directly — use `api_url()` plus the `get*`/`post*`/`patch*`/
  `put*`/`delete` helpers so error handling stays uniform.
- Avoid glob-import ambiguity between `leptos_router::*` and shared types in page modules —
  import concrete items.
- Run `cargo fmt --all` before committing.

### Frontend styling

- `crates/frontend/style.css` is the single design system. Sections in order:
  tokens, reset/base, layout, components, feature sections, utilities, responsive.
- Use design tokens (`var(--color-*)`, `var(--radius*)`) rather than raw hex/px. Component
  rules should contain no literal colors; if one is needed, add a token first. Inverted
  surfaces have their own scales (`--color-header-*`, `--color-log-*`, `--color-on-emphasis`)
  because they do not follow the light-canvas palette.
- Prefer existing utility and component classes over inline `style=` attributes.
- When adding markup with a new class, add its rule to `style.css` (there is no CSS
  framework — unstyled classes are a bug).
- Shared repo chrome (header + section nav) belongs in `components/repo_nav.rs` and should
  appear on every repository sub-page.

### Tests

- Backend integration tests live in `crates/backend/src/tests/`, one module per area plus
  a `mod.rs`, and exercise real handler/router code paths.
- Pure backend helpers get colocated `#[cfg(test)] mod tests` unit tests in the file that
  defines them, rather than being tested only through HTTP.
- Playwright specs live in `tests/` and run with `npx playwright test`. They use the
  `baseURL` from `playwright.config.ts` (`CODEZA_BASE_URL`, default
  `http://127.0.0.1:8080`) — never hardcode absolute URLs in specs; pass relative paths
  to `page.goto()` / `page.request.*()`.
- `tests/routes.spec.ts` is the route smoke net: keep its `ROUTES` list in sync with
  `crates/frontend/src/main.rs` so a renamed or removed route fails loudly.
- Ad-hoc API verification scripts live in `scripts/verify_*.py`.

## Verifying changes

```bash
cargo test --workspace
(cd crates/frontend && trunk build)
cargo run -p backend &                       # then:
python scripts/capture_screenshots.py docs/screenshots
python scripts/audit_responsive.py           # 390/768/1440 overflow sweep
```

Screenshot filenames are referenced by `README.md`; if you renumber routes in
`scripts/capture_screenshots.py`, update the README image links to match. Keep the route
list there pointing at resources that actually exist in the demo seed (for example the
seeded org slug is `codeza-org`, not `admin`).

## Gotchas

- In Axum, `.fallback()` replaces any previously set `.fallback_service()`. The static file
  handler in `main.rs` is intentionally a single `.fallback()` that resolves assets itself.
- The static handler serves a file only if its canonicalized real path stays inside the
  asset root (`read_within_root`). Normalizing `..` is not enough on its own: a symlink
  placed in `dist/` can point outside it, so keep the resolved-destination check when
  touching that code.
- The header hides the search box at ≤768px and shows a plain Search link (`.search-link`)
  instead; keep both in sync when changing the global nav.
- The frontend build output (`crates/frontend/dist/`) is gitignored — build it locally.
- `node_modules/` is gitignored; run `npm install` for Playwright.
- The detail endpoints return `200` with a JSON `null` body for a missing entity
  (`Json<Option<T>>`), *not* `404`. The frontend `get_opt` helper depends on this, so when
  adding a detail handler follow the same convention instead of returning `404`.
- At narrow viewports the global header scrolls horizontally (`overflow-x: auto`) rather
  than wrapping; the inner `.nav-links` deliberately overflow their container. Don't
  "fix" this by allowing the header to break out — it is what keeps the document from
  overflowing at 390px.

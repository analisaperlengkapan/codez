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
    routes.rs      HTTP route registration (grouped sub-routers)
    state.rs       Shared AppState (in-memory stores)
    handlers/      Feature handlers, one module per domain
      repo/        Repository handlers split by area (repos, issues, pulls,
                   comments, contents, webhooks, wiki, pulse) + shared helpers
    db.rs          Optional PostgreSQL layer
    seed.rs        Demo in-memory state
    tests.rs       Integration tests
crates/frontend    Leptos SPA
  src/
    main.rs        Router definitions
    api.rs         HTTP client helpers
    components/    Reusable UI (nav.rs, repo_nav.rs)
    pages/         One module per feature area
  style.css        Design system (tokens + components + utilities)
  index.html       Trunk entry (pulls in style.css)
crates/shared      DTOs and helpers shared by both crates
docs/screenshots   README gallery images
scripts/           Verification + screenshot tooling
tests/             Playwright end-to-end specs
```

## Build, test, lint

```bash
# Frontend must be built before the backend can serve it
(cd crates/frontend && trunk build)

cargo run -p backend                    # serve API + SPA on :3000
cargo test --workspace                  # all tests
cargo clippy --workspace --all-targets  # lint (keep this clean)
cargo fmt --all                         # format
```

CI (`.github/workflows/ci.yml`) runs the same checks on every push/PR: `cargo fmt
--check`, `cargo clippy -- -D warnings`, `cargo test`, a release `trunk build`, and the
Playwright e2e suite against a locally started backend + `trunk serve`.

The backend serves the SPA from `CODEZA_STATIC_DIR` (default `crates/frontend/dist`). Deep
links return `index.html`; unknown `/api/*` paths return `404`.

## Conventions

### Rust

- Keep `cargo clippy --workspace --all-targets` warning-free.
- One handler module per domain under `crates/backend/src/handlers/`. Large domains use a
  directory module (e.g. `handlers/repo/`) with a `mod.rs` that holds shared imports and
  helpers, and re-exports each submodule so callers keep using `handlers::<name>`.
- Frontend pages: one file per feature area under `crates/frontend/src/pages/`; large
  features (e.g. `repo`) use a directory module with a re-exporting `mod.rs`.
- Avoid glob-import ambiguity between `leptos_router::*` and shared types in page modules —
  import concrete items.
- Run `cargo fmt --all` before committing.

### Frontend styling

- `crates/frontend/style.css` is the single design system. Sections in order:
  tokens, reset/base, layout, components, feature sections, utilities, responsive.
- Use design tokens (`var(--color-*)`, `var(--radius*)`) rather than raw hex/px.
- Prefer existing utility and component classes over inline `style=` attributes.
- When adding markup with a new class, add its rule to `style.css` (there is no CSS
  framework — unstyled classes are a bug).
- Shared repo chrome (header + section nav) belongs in `components/repo_nav.rs` and should
  appear on every repository sub-page.

### Tests

- Backend integration tests live in `crates/backend/src/tests.rs` and exercise real
  handler/router code paths.
- Playwright specs live in `tests/` and run with `npx playwright test`. They use the
  `baseURL` from `playwright.config.ts` (`CODEZA_BASE_URL`, default
  `http://127.0.0.1:8080`) — never hardcode absolute URLs in specs; pass relative paths
  to `page.goto()` / `page.request.*()`.
- Ad-hoc API verification scripts live in `scripts/verify_*.py`.

## Verifying changes

```bash
cargo test --workspace
(cd crates/frontend && trunk build)
cargo run -p backend &                       # then:
python scripts/capture_screenshots.py docs/screenshots
```

Screenshot filenames are referenced by `README.md`; if you renumber routes in
`scripts/capture_screenshots.py`, update the README image links to match.

## Gotchas

- In Axum, `.fallback()` replaces any previously set `.fallback_service()`. The static file
  handler in `main.rs` is intentionally a single `.fallback()` that resolves assets itself.
- The frontend build output (`crates/frontend/dist/`) is gitignored — build it locally.
- `node_modules/` is gitignored; run `npm install` for Playwright.

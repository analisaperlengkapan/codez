# Codez

Codez is a lightweight, Gitea-inspired forge platform built with Rust. It features a modern stack with a backend API service and a frontend web application.

## Tech Stack

- **Backend:** [Axum](https://github.com/tokio-rs/axum) (Rust) - High performance, asynchronous web framework.
- **Frontend:** [Leptos](https://github.com/leptos-rs/leptos) (Rust) - Reactive web framework for building web apps in WebAssembly.
- **Shared Library:** A common crate sharing data models and types between backend and frontend.
- **Persistence:** Currently implemented with in-memory storage (thread-safe `Arc<RwLock>`) for rapid prototyping and development.

## Features

The platform provides a comprehensive set of features for software development collaboration:

- **Repositories:** Create, list, fork, and manage repositories.
  - Supports repository settings (default branch, merge strategies, feature toggles).
  - File browsing and raw file access.
  - Collaborator management (add/remove users).
- **Issues & Pull Requests:**
  - Create and manage issues and pull requests.
  - Comments, reactions, and assignments.
  - Milestones and labels.
- **Organizations & Teams:**
  - Create organizations and teams.
  - Manage membership and visibility.
- **Security:**
  - **Secrets:** Manage repository secrets.
  - **Deploy Keys:** Manage SSH deploy keys.
- **CI/CD & Webhooks:**
  - Workflow runs and actions triggers.
  - Webhook management and delivery logging.
- **User Management:**
  - Registration, login, and user profiles.
  - SSH and GPG key management.
  - Activity feeds and notifications.

## Project Structure

```
.
├── crates/
│   ├── backend/    # Axum-based API server
│   ├── frontend/   # Leptos-based WASM frontend
│   └── shared/     # Shared Rust structs and types
├── Cargo.toml      # Workspace configuration
└── README.md
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Trunk](https://trunkrs.dev/) (for frontend): `cargo install trunk`

### Running the Backend

The backend listens on port `3000` by default.

```bash
cargo run -p backend
```

### Running the Frontend

The frontend is served via Trunk, defaulting to port `8080`.

```bash
cd crates/frontend
trunk serve
```

## API Documentation

The backend exposes a RESTful API at `/api/v1`. Key endpoints include:

- `GET /api/v1/repos`: List repositories.
- `GET /api/v1/repos/:owner/:repo`: Get repository details.
- `POST /api/v1/user/repos`: Create a new repository.
- `GET /api/v1/orgs`: Manage organizations.

(See `crates/backend/src/router.rs` for the full routing table.)

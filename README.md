# Codeza

Codeza is a lightweight, Gitea-inspired code hosting platform built with Rust. It features a modern, full-stack architecture using **Axum** for the backend and **Leptos** for the frontend.

**Note:** This project currently uses in-memory persistence for demonstration and development purposes. Data is lost when the backend server restarts.

## Features

Codeza implements a wide range of GitHub/Gitea-compatible features:

*   **Repositories**: Create, list, fork, transfer, and configure repositories.
*   **Issues & Pull Requests**: Full lifecycle management with comments, labels, milestones, and assignees.
*   **Discussions**: Community discussions with categories and comments.
*   **Releases & Packages**: Manage software releases with assets and package registry support.
*   **Organizations & Teams**: Manage organizations, teams, and membership.
*   **User Management**: Authentication, profile settings, SSH/GPG keys, and 2FA settings.
*   **Social**: Star and watch repositories, follow users, and activity feeds.
*   **Developer Settings**: Manage OAuth2 applications and personal access tokens.
*   **Wiki**: Integrated wiki for documentation.
*   **Git LFS**: Basic support for Git Large File Storage locks.
*   **Webhooks**: Repository webhooks with delivery history.
*   **Secrets & Keys**: Manage repository secrets and deploy keys.

## Tech Stack

*   **Backend**: [Axum](https://github.com/tokio-rs/axum) (Rust)
*   **Frontend**: [Leptos](https://github.com/leptos-rs/leptos) (Rust/WASM)
*   **Shared**: Common logic and data structures shared between backend and frontend.
*   **Build Tool**: Cargo workspaces.

## Getting Started

### Prerequisites

*   Rust (latest stable)
*   `trunk` (for frontend building): `cargo install trunk`
*   `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Running the Application

1.  **Start the Backend:**
    ```bash
    cargo run -p backend
    ```
    The API server will start at `http://127.0.0.1:3000`.

2.  **Start the Frontend:**
    ```bash
    trunk serve
    ```
    The application will be available at `http://127.0.0.1:8080`.

## Development

*   **Linting**: `cargo clippy --workspace -- -D warnings`
*   **Formatting**: `cargo fmt --all`
*   **Testing**: `cargo test -p shared` (and `backend`/`frontend`)

## Project Structure

*   `crates/backend`: Axum-based API server.
*   `crates/frontend`: Leptos web application.
*   `crates/shared`: Shared types (DTOs) and utilities.

## API Documentation

API endpoints are prefixed with `/api/v1` and follow standard REST conventions similar to GitHub/Gitea APIs.

//! HTTP handlers, one module per domain.
//!
//! Each domain exposes `pub async fn` handlers that take Axum extractors and
//! return JSON. The larger repository surface lives in the `repo` directory
//! module, split by area. Every handler is re-exported here so the router can
//! import them from a single path (`crate::handlers::*`).

pub mod action;
pub mod discussion;
pub mod org;
pub mod package;
pub mod project;
pub mod release;
pub mod repo;
pub mod user;

pub use action::*;
pub use discussion::*;
pub use org::*;
pub use package::*;
pub use project::*;
pub use release::*;
pub use repo::*;
pub use user::*;

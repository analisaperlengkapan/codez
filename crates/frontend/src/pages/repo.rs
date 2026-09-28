//! Repository pages, split by feature area.

pub mod create;
pub mod issues;
pub mod pulls;
pub mod repo_detail;
pub mod settings;
pub mod wiki;

pub use create::*;
pub use issues::*;
pub use pulls::*;
pub use repo_detail::*;
pub use settings::*;
pub use wiki::*;

//! Codeza MicroFrontend Manager
//! Handles MFE orchestration, module federation, and dynamic loading

pub mod db;
pub mod federation;
pub mod loader;
pub mod mfe;

pub use db::MFERepository;
pub use federation::{ExposeConfig, FederationConfig, RemoteConfig, SharedConfig};
pub use loader::{InMemoryModuleLoader, LoadedModule, ModuleLoader, RemoteModuleLoader};
pub use mfe::{MFERegistry, MFEStatus, MicroFrontend, SharedDependency};

//! Codeza MicroFrontend Manager
//! Handles MFE orchestration, module federation, and dynamic loading

pub mod mfe;
pub mod federation;
pub mod loader;

pub use mfe::{MicroFrontend, MFEStatus, MFERegistry, SharedDependency};
pub use federation::{FederationConfig, RemoteConfig, ExposeConfig, SharedConfig};
pub use loader::{ModuleLoader, InMemoryModuleLoader, RemoteModuleLoader, LoadedModule};

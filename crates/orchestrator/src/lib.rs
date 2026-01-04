//! Codeza SuperApp Orchestrator
//! Handles SuperApp composition, state management, routing, and event bus

pub mod superapp;
pub mod state;
pub mod routing;
pub mod event;
pub mod db;

pub use superapp::{SuperApp, AppModule, AppConfig};
pub use state::StateStore;
pub use routing::{Router, Route, NavigationEvent};
pub use event::{Event, EventBus};
pub use db::SuperAppRepository;

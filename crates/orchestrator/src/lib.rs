//! Codeza SuperApp Orchestrator
//! Handles SuperApp composition, state management, routing, and event bus

pub mod db;
pub mod event;
pub mod routing;
pub mod service;
pub mod state;
pub mod superapp;

pub use db::SuperAppRepository;
pub use event::{Event, EventBus};
pub use routing::{NavigationEvent, Route, Router};
pub use service::generate_manifest;
pub use state::StateStore;
pub use superapp::{AppConfig, AppModule, SuperApp};

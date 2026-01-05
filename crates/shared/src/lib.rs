//! Codeza Shared Library
//! Common utilities, error handling, and middleware for all services

pub mod alerting;
pub mod analytics;
pub mod analytics_api;
pub mod auth;
pub mod auth_middleware;
mod auth_test;
pub mod config;
pub mod dashboard;
pub mod error;
pub mod logging;
pub mod metrics;
pub mod middleware;
pub mod models;
pub mod report_generator;
pub mod tracing_module;

#[cfg(test)]
mod analytics_tests;

pub use alerting::{Alert, AlertManager, AlertRule};
pub use analytics::{
    AnalyticsEngine, PipelineAnalytics, Report, RepositoryAnalytics, UserActivity,
};
pub use analytics_api::{AnalyticsQuery, AnalyticsResponse, QueryBuilder, QueryExecutor};
pub use auth::*;
pub use config::Config;
pub use dashboard::{Dashboard, DashboardPermission, DashboardService, WidgetConfig};
pub use error::{CodezaError, Result};
pub use logging::init_logging;
pub use metrics::{Counter, Gauge, Histogram, MetricsRegistry};
pub use models::*;
pub use report_generator::{ExportFormat, GeneratedReport, ReportGeneratorService, ReportTemplate};
pub use tracing_module::{Span, Trace, Tracer};

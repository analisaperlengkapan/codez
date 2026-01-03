//! Codeza Shared Library
//! Common utilities, error handling, and middleware for all services

pub mod error;
pub mod config;
pub mod logging;
pub mod middleware;
pub mod models;
pub mod auth;
pub mod auth_middleware;
mod auth_test;
pub mod metrics;
pub mod tracing_module;
pub mod alerting;
pub mod analytics;
pub mod dashboard;
pub mod report_generator;
pub mod analytics_api;

#[cfg(test)]
mod analytics_tests;

pub use error::{CodezaError, Result};
pub use config::Config;
pub use logging::init_logging;
pub use models::*;
pub use auth::*;
pub use metrics::{MetricsRegistry, Counter, Gauge, Histogram};
pub use tracing_module::{Tracer, Span, Trace};
pub use alerting::{AlertManager, AlertRule, Alert};
pub use analytics::{AnalyticsEngine, RepositoryAnalytics, PipelineAnalytics, UserActivity, Report};
pub use dashboard::{Dashboard, DashboardService, WidgetConfig, DashboardPermission};
pub use report_generator::{ReportGeneratorService, ReportTemplate, GeneratedReport, ExportFormat};
pub use analytics_api::{QueryBuilder, QueryExecutor, AnalyticsQuery, AnalyticsResponse};

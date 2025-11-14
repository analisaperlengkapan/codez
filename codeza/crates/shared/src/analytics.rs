//! Advanced analytics and reporting for Codeza Platform

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Repository analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryAnalytics {
    pub repository_id: Uuid,
    pub total_commits: u64,
    pub total_contributors: u32,
    pub total_branches: u32,
    pub total_tags: u32,
    pub lines_of_code: u64,
    pub commit_frequency: f64, // commits per day
    pub contributor_growth: f64, // growth rate
    pub last_commit: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Pipeline analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineAnalytics {
    pub pipeline_id: Uuid,
    pub total_runs: u64,
    pub successful_runs: u64,
    pub failed_runs: u64,
    pub success_rate: f64,
    pub average_duration: u64, // seconds
    pub median_duration: u64,
    pub failure_rate: f64,
    pub trend: AnalyticsTrend,
    pub last_run: Option<chrono::DateTime<chrono::Utc>>,
}

/// User activity analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub user_id: Uuid,
    pub total_logins: u64,
    pub total_commits: u64,
    pub total_pull_requests: u64,
    pub total_issues: u64,
    pub last_active: chrono::DateTime<chrono::Utc>,
    pub activity_score: f64,
    pub contribution_streak: u32,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub service_name: String,
    pub average_response_time: f64, // milliseconds
    pub p95_response_time: f64,
    pub p99_response_time: f64,
    pub throughput: f64, // requests per second
    pub error_rate: f64,
    pub uptime: f64, // percentage
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Analytics trend
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalyticsTrend {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// Dashboard widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub title: String,
    pub widget_type: WidgetType,
    pub data: serde_json::Value,
    pub position: (u32, u32),
    pub size: (u32, u32),
}

/// Widget type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetType {
    LineChart,
    BarChart,
    PieChart,
    Gauge,
    Stat,
    Table,
}

/// Custom report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub report_type: ReportType,
    pub filters: HashMap<String, String>,
    pub widgets: Vec<DashboardWidget>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Report type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportType {
    Repository,
    Pipeline,
    User,
    Performance,
    Custom,
}

/// Analytics engine
pub struct AnalyticsEngine {
    repository_analytics: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, RepositoryAnalytics>>>,
    pipeline_analytics: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, PipelineAnalytics>>>,
    user_activity: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, UserActivity>>>,
    performance_metrics: std::sync::Arc<tokio::sync::RwLock<Vec<PerformanceMetrics>>>,
}

impl AnalyticsEngine {
    /// Create new analytics engine
    pub fn new() -> Self {
        Self {
            repository_analytics: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            pipeline_analytics: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            user_activity: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            performance_metrics: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// Record repository analytics
    pub async fn record_repository(&self, analytics: RepositoryAnalytics) {
        let mut repo_analytics = self.repository_analytics.write().await;
        repo_analytics.insert(analytics.repository_id, analytics);
    }

    /// Get repository analytics
    pub async fn get_repository(&self, repository_id: Uuid) -> Option<RepositoryAnalytics> {
        let repo_analytics = self.repository_analytics.read().await;
        repo_analytics.get(&repository_id).cloned()
    }

    /// Record pipeline analytics
    pub async fn record_pipeline(&self, analytics: PipelineAnalytics) {
        let mut pipeline_analytics = self.pipeline_analytics.write().await;
        pipeline_analytics.insert(analytics.pipeline_id, analytics);
    }

    /// Get pipeline analytics
    pub async fn get_pipeline(&self, pipeline_id: Uuid) -> Option<PipelineAnalytics> {
        let pipeline_analytics = self.pipeline_analytics.read().await;
        pipeline_analytics.get(&pipeline_id).cloned()
    }

    /// Record user activity
    pub async fn record_user_activity(&self, activity: UserActivity) {
        let mut user_activity = self.user_activity.write().await;
        user_activity.insert(activity.user_id, activity);
    }

    /// Get user activity
    pub async fn get_user_activity(&self, user_id: Uuid) -> Option<UserActivity> {
        let user_activity = self.user_activity.read().await;
        user_activity.get(&user_id).cloned()
    }

    /// Record performance metrics
    pub async fn record_performance(&self, metrics: PerformanceMetrics) {
        let mut perf_metrics = self.performance_metrics.write().await;
        perf_metrics.push(metrics);
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> Vec<PerformanceMetrics> {
        self.performance_metrics.read().await.clone()
    }

    /// Get all repository analytics
    pub async fn all_repositories(&self) -> Vec<RepositoryAnalytics> {
        let repo_analytics = self.repository_analytics.read().await;
        repo_analytics.values().cloned().collect()
    }

    /// Get all pipeline analytics
    pub async fn all_pipelines(&self) -> Vec<PipelineAnalytics> {
        let pipeline_analytics = self.pipeline_analytics.read().await;
        pipeline_analytics.values().cloned().collect()
    }
}

impl Default for AnalyticsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Report {
    /// Create new report
    pub fn new(name: String, report_type: ReportType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            report_type,
            filters: HashMap::new(),
            widgets: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Add widget to report
    pub fn add_widget(&mut self, widget: DashboardWidget) {
        self.widgets.push(widget);
        self.updated_at = chrono::Utc::now();
    }

    /// Add filter
    pub fn add_filter(&mut self, key: String, value: String) {
        self.filters.insert(key, value);
        self.updated_at = chrono::Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analytics_engine() {
        let engine = AnalyticsEngine::new();
        let repo_id = Uuid::new_v4();

        let analytics = RepositoryAnalytics {
            repository_id: repo_id,
            total_commits: 100,
            total_contributors: 5,
            total_branches: 10,
            total_tags: 3,
            lines_of_code: 5000,
            commit_frequency: 2.5,
            contributor_growth: 0.1,
            last_commit: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        };

        engine.record_repository(analytics.clone()).await;
        let retrieved = engine.get_repository(repo_id).await;

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().total_commits, 100);
    }

    #[test]
    fn test_report_creation() {
        let mut report = Report::new("Test Report".to_string(), ReportType::Repository);
        report.add_filter("repository".to_string(), "repo-1".to_string());

        assert_eq!(report.name, "Test Report");
        assert_eq!(report.filters.len(), 1);
    }
}

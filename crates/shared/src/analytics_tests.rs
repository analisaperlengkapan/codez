//! Comprehensive tests for Phase 11: Advanced Analytics & Reporting

#[cfg(test)]
mod integration_tests {
    use crate::analytics::*;
    use crate::dashboard::*;
    use crate::report_generator::*;
    use crate::analytics_api::*;
    use uuid::Uuid;

    // ============= Analytics Engine Tests =============

    #[tokio::test]
    async fn test_analytics_engine_repository_tracking() {
        let engine = AnalyticsEngine::new();
        let repo_id = Uuid::new_v4();

        let analytics = RepositoryAnalytics {
            repository_id: repo_id,
            total_commits: 150,
            total_contributors: 8,
            total_branches: 15,
            total_tags: 5,
            lines_of_code: 10000,
            commit_frequency: 3.5,
            contributor_growth: 0.15,
            last_commit: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        };

        engine.record_repository(analytics.clone()).await;
        let retrieved = engine.get_repository(repo_id).await;

        assert!(retrieved.is_some());
        let repo = retrieved.unwrap();
        assert_eq!(repo.total_commits, 150);
        assert_eq!(repo.total_contributors, 8);
    }

    #[tokio::test]
    async fn test_analytics_engine_pipeline_tracking() {
        let engine = AnalyticsEngine::new();
        let pipeline_id = Uuid::new_v4();

        let analytics = PipelineAnalytics {
            pipeline_id,
            total_runs: 100,
            successful_runs: 95,
            failed_runs: 5,
            success_rate: 0.95,
            average_duration: 300,
            median_duration: 280,
            failure_rate: 0.05,
            trend: AnalyticsTrend::Increasing,
            last_run: Some(chrono::Utc::now()),
        };

        engine.record_pipeline(analytics.clone()).await;
        let retrieved = engine.get_pipeline(pipeline_id).await;

        assert!(retrieved.is_some());
        let pipeline = retrieved.unwrap();
        assert_eq!(pipeline.success_rate, 0.95);
        assert_eq!(pipeline.trend, AnalyticsTrend::Increasing);
    }

    #[tokio::test]
    async fn test_analytics_engine_user_activity() {
        let engine = AnalyticsEngine::new();
        let user_id = Uuid::new_v4();

        let activity = UserActivity {
            user_id,
            total_logins: 50,
            total_commits: 25,
            total_pull_requests: 10,
            total_issues: 5,
            last_active: chrono::Utc::now(),
            activity_score: 85.5,
            contribution_streak: 15,
        };

        engine.record_user_activity(activity.clone()).await;
        let retrieved = engine.get_user_activity(user_id).await;

        assert!(retrieved.is_some());
        let user = retrieved.unwrap();
        assert_eq!(user.total_commits, 25);
        assert_eq!(user.contribution_streak, 15);
    }

    #[tokio::test]
    async fn test_analytics_engine_all_repositories() {
        let engine = AnalyticsEngine::new();

        for i in 0..5 {
            let analytics = RepositoryAnalytics {
                repository_id: Uuid::new_v4(),
                total_commits: 100 + i,
                total_contributors: 5,
                total_branches: 10,
                total_tags: 3,
                lines_of_code: 5000,
                commit_frequency: 2.5,
                contributor_growth: 0.1,
                last_commit: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
            };
            engine.record_repository(analytics).await;
        }

        let all = engine.all_repositories().await;
        assert_eq!(all.len(), 5);
    }

    // ============= Dashboard Service Tests =============

    #[tokio::test]
    async fn test_dashboard_creation() {
        let service = DashboardService::new();
        let owner_id = Uuid::new_v4();

        let result = service
            .create_dashboard("Analytics Dashboard".to_string(), owner_id)
            .await;

        assert!(result.is_ok());
        let dashboard = result.unwrap();
        assert_eq!(dashboard.name, "Analytics Dashboard");
        assert_eq!(dashboard.owner_id, owner_id);
        assert!(!dashboard.is_public);
    }

    #[tokio::test]
    async fn test_dashboard_widget_management() {
        let service = DashboardService::new();
        let owner_id = Uuid::new_v4();

        let dashboard = service
            .create_dashboard("Test Dashboard".to_string(), owner_id)
            .await
            .unwrap();

        let widget1 = WidgetConfig {
            id: Uuid::new_v4(),
            widget_type: "LineChart".to_string(),
            title: "Commits Trend".to_string(),
            position: (0, 0),
            size: (6, 4),
            config: std::collections::HashMap::new(),
            data_source: Some("repository_commits".to_string()),
        };

        let widget2 = WidgetConfig {
            id: Uuid::new_v4(),
            widget_type: "BarChart".to_string(),
            title: "Pipeline Success Rate".to_string(),
            position: (6, 0),
            size: (6, 4),
            config: std::collections::HashMap::new(),
            data_source: Some("pipeline_success".to_string()),
        };

        service.add_widget(dashboard.id, widget1).await.unwrap();
        service.add_widget(dashboard.id, widget2).await.unwrap();

        let updated = service.get_dashboard(dashboard.id).await.unwrap();
        assert_eq!(updated.widgets.len(), 2);
    }

    #[tokio::test]
    async fn test_dashboard_versioning() {
        let service = DashboardService::new();
        let owner_id = Uuid::new_v4();

        let dashboard = service
            .create_dashboard("Versioned Dashboard".to_string(), owner_id)
            .await
            .unwrap();

        service.save_version(dashboard.id, owner_id).await.unwrap();
        service.save_version(dashboard.id, owner_id).await.unwrap();

        let versions = service.get_versions(dashboard.id).await;
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, 1);
        assert_eq!(versions[1].version, 2);
    }

    // ============= Report Generator Tests =============

    #[tokio::test]
    async fn test_report_template_creation() {
        let service = ReportGeneratorService::new();
        let template = ReportTemplate {
            id: Uuid::new_v4(),
            name: "Monthly Report".to_string(),
            description: Some("Monthly analytics report".to_string()),
            template_type: ReportTemplateType::Repository,
            sections: Vec::new(),
            parameters: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
        };

        let result = service.create_template(template.clone()).await;
        assert!(result.is_ok());

        let retrieved = service.get_template(template.id).await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().name, "Monthly Report");
    }

    #[tokio::test]
    async fn test_report_generation_multiple_formats() {
        let service = ReportGeneratorService::new();
        let template = ReportTemplate {
            id: Uuid::new_v4(),
            name: "Test Report".to_string(),
            description: None,
            template_type: ReportTemplateType::Pipeline,
            sections: Vec::new(),
            parameters: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
        };

        service.create_template(template.clone()).await.unwrap();

        let pdf_report = service
            .generate_report(template.id, ExportFormat::PDF, Uuid::new_v4())
            .await
            .unwrap();
        assert_eq!(pdf_report.format, ExportFormat::PDF);

        let excel_report = service
            .generate_report(template.id, ExportFormat::Excel, Uuid::new_v4())
            .await
            .unwrap();
        assert_eq!(excel_report.format, ExportFormat::Excel);

        let csv_report = service
            .generate_report(template.id, ExportFormat::CSV, Uuid::new_v4())
            .await
            .unwrap();
        assert_eq!(csv_report.format, ExportFormat::CSV);
    }

    #[tokio::test]
    async fn test_report_export() {
        let service = ReportGeneratorService::new();
        let template = ReportTemplate {
            id: Uuid::new_v4(),
            name: "Export Test".to_string(),
            description: None,
            template_type: ReportTemplateType::Custom,
            sections: Vec::new(),
            parameters: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
        };

        service.create_template(template.clone()).await.unwrap();

        let report = service
            .generate_report(template.id, ExportFormat::JSON, Uuid::new_v4())
            .await
            .unwrap();

        let exported = service
            .export_report(report.id, ExportFormat::JSON)
            .await;

        assert!(exported.is_ok());
        let data = exported.unwrap();
        assert!(!data.is_empty());
    }

    #[tokio::test]
    async fn test_report_scheduling() {
        let service = ReportGeneratorService::new();

        let schedule = ReportSchedule {
            id: Uuid::new_v4(),
            template_id: Uuid::new_v4(),
            frequency: ScheduleFrequency::Weekly,
            distribution_channels: vec![DistributionChannel {
                channel_type: ChannelType::Email,
                destination: "team@example.com".to_string(),
            }],
            enabled: true,
            created_at: chrono::Utc::now(),
        };

        let result = service.create_schedule(schedule).await;
        assert!(result.is_ok());

        let schedules = service.get_schedules().await;
        assert_eq!(schedules.len(), 1);
    }

    // ============= Analytics API Tests =============

    #[test]
    fn test_query_builder_fluent_api() {
        let query = QueryBuilder::new(EntityType::Repository)
            .filter("status".to_string(), FilterOperator::Equals, "active".to_string())
            .filter("created_at".to_string(), FilterOperator::GreaterThan, "2025-01-01".to_string())
            .sort("updated_at".to_string(), SortOrder::Descending)
            .paginate(25, 50)
            .build();

        assert_eq!(query.entity_type, EntityType::Repository);
        assert_eq!(query.filters.len(), 2);
        assert_eq!(query.pagination.limit, 25);
        assert_eq!(query.pagination.offset, 50);
    }

    #[tokio::test]
    async fn test_query_executor_pagination() {
        let query = QueryBuilder::new(EntityType::Pipeline)
            .paginate(10, 0)
            .build();

        let data: Vec<i32> = (1..=100).collect();
        let response = QueryExecutor::execute(query, data).await;

        assert_eq!(response.data.len(), 10);
        assert_eq!(response.pagination.total, 100);
        assert_eq!(response.pagination.pages, 10);
        assert_eq!(response.pagination.offset, 0);
    }

    #[tokio::test]
    async fn test_query_executor_pagination_offset() {
        let query = QueryBuilder::new(EntityType::User)
            .paginate(10, 50)
            .build();

        let data: Vec<i32> = (1..=100).collect();
        let response = QueryExecutor::execute(query, data).await;

        assert_eq!(response.data.len(), 10);
        assert_eq!(response.data[0], 51);
        assert_eq!(response.pagination.offset, 50);
    }

    // ============= Integration Workflow Tests =============

    #[tokio::test]
    async fn test_end_to_end_analytics_workflow() {
        // Create analytics engine
        let analytics_engine = AnalyticsEngine::new();
        let repo_id = Uuid::new_v4();

        // Record repository analytics
        let repo_analytics = RepositoryAnalytics {
            repository_id: repo_id,
            total_commits: 200,
            total_contributors: 10,
            total_branches: 20,
            total_tags: 5,
            lines_of_code: 15000,
            commit_frequency: 4.0,
            contributor_growth: 0.2,
            last_commit: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        };
        analytics_engine.record_repository(repo_analytics).await;

        // Create dashboard
        let dashboard_service = DashboardService::new();
        let owner_id = Uuid::new_v4();
        let dashboard = dashboard_service
            .create_dashboard("Analytics Dashboard".to_string(), owner_id)
            .await
            .unwrap();

        // Add widgets
        let widget = WidgetConfig {
            id: Uuid::new_v4(),
            widget_type: "LineChart".to_string(),
            title: "Repository Metrics".to_string(),
            position: (0, 0),
            size: (12, 6),
            config: std::collections::HashMap::new(),
            data_source: Some("repository".to_string()),
        };
        dashboard_service
            .add_widget(dashboard.id, widget)
            .await
            .unwrap();

        // Generate report
        let report_service = ReportGeneratorService::new();
        let template = ReportTemplate {
            id: Uuid::new_v4(),
            name: "Analytics Report".to_string(),
            description: None,
            template_type: ReportTemplateType::Repository,
            sections: Vec::new(),
            parameters: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
        };
        report_service.create_template(template.clone()).await.unwrap();

        let report = report_service
            .generate_report(template.id, ExportFormat::PDF, owner_id)
            .await
            .unwrap();

        // Verify workflow
        let retrieved_analytics = analytics_engine.get_repository(repo_id).await;
        let retrieved_dashboard = dashboard_service.get_dashboard(dashboard.id).await;
        let retrieved_report = report_service.get_report(report.id).await;

        assert!(retrieved_analytics.is_some());
        assert!(retrieved_dashboard.is_ok());
        assert!(retrieved_report.is_ok());
    }

    // ============= Edge Case Tests =============

    #[tokio::test]
    async fn test_dashboard_delete_nonexistent() {
        let service = DashboardService::new();
        let result = service.delete_dashboard(Uuid::new_v4()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_report_export_nonexistent() {
        let service = ReportGeneratorService::new();
        let result = service
            .export_report(Uuid::new_v4(), ExportFormat::PDF)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_query_executor_empty_data() {
        let query = QueryBuilder::new(EntityType::Repository)
            .paginate(10, 0)
            .build();

        let data: Vec<i32> = Vec::new();
        let response = QueryExecutor::execute(query, data).await;

        assert_eq!(response.data.len(), 0);
        assert_eq!(response.pagination.total, 0);
        assert_eq!(response.pagination.pages, 0);
    }

    // ============= Performance Tests =============

    #[tokio::test]
    async fn test_analytics_engine_performance_large_dataset() {
        let engine = AnalyticsEngine::new();

        // Record 1000 repositories
        for i in 0..1000 {
            let analytics = RepositoryAnalytics {
                repository_id: Uuid::new_v4(),
                total_commits: 100 + i,
                total_contributors: 5,
                total_branches: 10,
                total_tags: 3,
                lines_of_code: 5000,
                commit_frequency: 2.5,
                contributor_growth: 0.1,
                last_commit: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
            };
            engine.record_repository(analytics).await;
        }

        let all = engine.all_repositories().await;
        assert_eq!(all.len(), 1000);
    }

    #[tokio::test]
    async fn test_query_executor_performance_large_dataset() {
        let query = QueryBuilder::new(EntityType::Repository)
            .paginate(100, 0)
            .build();

        let data: Vec<i32> = (1..=10000).collect();
        let response = QueryExecutor::execute(query, data).await;

        assert_eq!(response.data.len(), 100);
        assert_eq!(response.pagination.total, 10000);
        assert!(response.query_time_ms < 100); // Should be fast
    }
}

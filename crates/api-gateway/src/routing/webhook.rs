use super::AppState;
use crate::routing::git::build_git_provider_config;
use codeza_cicd_engine::{
    GitPushContext, LocalJobExecutor, PipelineExecutionRepository, trigger_push_pipeline,
    trigger_push_pipeline_from_yaml,
};
use codeza_git_service::create_git_provider;

pub async fn git_webhook(
    axum::extract::State(state): axum::extract::State<AppState>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    let provider_str = state.config.git.provider.to_lowercase();

    let span = tracing::span!(
        tracing::Level::INFO,
        "git_webhook",
        provider = %provider_str,
    );
    let _enter = span.enter();

    // Count all webhook hits per provider
    let counter_name = format!("git_webhook_events_total.{}", provider_str);
    state.metrics.register_counter(counter_name).inc();

    match provider_str.as_str() {
        "gitea" => handle_gitea_webhook(state.clone(), headers, body).await,
        "gitlab" => handle_gitlab_webhook(state.clone(), headers, body).await,
        "github" => handle_github_webhook(state.clone(), headers, body).await,
        other => Err(codeza_shared::CodezaError::ConfigError(format!(
            "Unsupported GIT_PROVIDER value for webhook: {}",
            other
        ))),
    }
}

async fn handle_github_webhook(
    state: AppState,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    let secret = state.config.git.webhook_secret.clone();
    let repo = PipelineExecutionRepository::new(state.pool.clone());

    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            codeza_shared::CodezaError::AuthenticationError(
                "Missing X-Hub-Signature-256 header".to_string(),
            )
        })?;

    // GitHub signature is "sha256=..."
    let validator = codeza_git_service::WebhookValidator::new(secret);
    if !validator.validate(&body, signature) {
        return Err(codeza_shared::CodezaError::AuthenticationError(
            "Invalid GitHub webhook signature".to_string(),
        ));
    }

    let event_header = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Minimal structs for GitHub payload parsing
    #[derive(Debug, serde::Deserialize)]
    struct GitHubUser {
        name: String,
        email: Option<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct GitHubRepo {
        full_name: String,
    }

    #[derive(Debug, serde::Deserialize)]
    struct GitHubPushEvent {
        #[serde(rename = "ref")]
        ref_: String,
        after: String,
        repository: GitHubRepo,
        pusher: GitHubUser,
    }

    match event_header {
        "push" => {
            if let Ok(event) = serde_json::from_slice::<GitHubPushEvent>(&body) {
                tracing::info!(
                    target = "git-webhook",
                    "Received GitHub push event on repo {} by {} (ref {}, after {})",
                    event.repository.full_name,
                    event.pusher.name,
                    event.ref_,
                    event.after,
                );

                let ctx = GitPushContext {
                    provider: "github".to_string(),
                    repo: event.repository.full_name.clone(),
                    r#ref: event.ref_.clone(),
                    after: event.after.clone(),
                };

                let executor = LocalJobExecutor;
                let provider_config = build_git_provider_config(&state.config)?;
                let provider = create_git_provider(provider_config);

                let yaml = codeza_cicd_engine::load_yaml_pipeline_config(
                    provider.as_ref(),
                    &event.repository.full_name,
                    &event.ref_,
                )
                .await;

                let trigger_result = if let Some(yaml) = yaml {
                    trigger_push_pipeline_from_yaml(&executor, &ctx, &yaml).await
                } else {
                    trigger_push_pipeline(&executor, &ctx).await
                };
                let pipeline = trigger_result.pipeline;

                state
                    .metrics
                    .register_counter("ci_pipelines_triggered_total.github".to_string())
                    .inc();

                tracing::info!(
                    target = "cicd-trigger",
                    pipeline_id = %pipeline.id,
                    repo = %event.repository.full_name,
                    r#ref = %event.ref_,
                    "Created CI/CD pipeline stub from GitHub push",
                );

                if let Err(e) = repo
                    .create_execution(
                        "github",
                        &event.repository.full_name,
                        &event.ref_,
                        &event.after,
                        pipeline.id,
                    )
                    .await
                {
                    tracing::warn!("Failed to persist pipeline execution: {}", e);
                }

                if let Some(execution) = trigger_result.job_execution {
                    tracing::info!(
                        target = "cicd-exec",
                        job_name = %execution.name,
                        status = ?execution.status,
                        duration = ?execution.duration,
                        "Executed CI/CD job from GitHub push",
                    );

                    if let Err(e) = repo
                        .create_job_execution(
                            pipeline.id,
                            "github",
                            &event.repository.full_name,
                            &event.ref_,
                            &event.after,
                            &execution,
                        )
                        .await
                    {
                        tracing::warn!("Failed to persist job execution: {}", e);
                    }
                }
            } else {
                 tracing::warn!(
                    target = "git-webhook",
                    "Failed to parse GitHub push payload",
                );
            }
        }
        _ => {
            tracing::info!(
                target = "git-webhook",
                "Received GitHub webhook event {}: {}",
                event_header,
                String::from_utf8_lossy(&body),
            );
        }
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn handle_gitea_webhook(
    state: AppState,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    let secret = state.config.git.webhook_secret.clone();
    let repo = PipelineExecutionRepository::new(state.pool.clone());

    let signature = headers
        .get("X-Gitea-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            codeza_shared::CodezaError::AuthenticationError(
                "Missing X-Gitea-Signature header".to_string(),
            )
        })?;

    let validator = codeza_git_service::WebhookValidator::new(secret);

    if !validator.validate(&body, signature) {
        return Err(codeza_shared::CodezaError::AuthenticationError(
            "Invalid Gitea webhook signature".to_string(),
        ));
    }

    let event_header = headers
        .get("X-Gitea-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let event_type = match event_header {
        "push" => Some(codeza_git_service::WebhookEventType::Push),
        "pull_request" => Some(codeza_git_service::WebhookEventType::PullRequest),
        "issues" => Some(codeza_git_service::WebhookEventType::Issue),
        "release" => Some(codeza_git_service::WebhookEventType::Release),
        "repository" => Some(codeza_git_service::WebhookEventType::Repository),
        _ => None,
    };

    if let Some(event_type) = event_type {
        // Count webhook events by type for Gitea
        let event_counter_name = format!("git_webhook_events_total.gitea.{}", event_header);
        state.metrics.register_counter(event_counter_name).inc();

        match event_type {
            codeza_git_service::WebhookEventType::Push => {
                if let Ok(event) = serde_json::from_slice::<codeza_git_service::PushEvent>(&body) {
                    tracing::info!(
                        target = "git-webhook",
                        "Received Gitea push event on repo {} by {}",
                        event.repository.full_name,
                        event.pusher.username,
                    );

                    let ctx = GitPushContext {
                        provider: "gitea".to_string(),
                        repo: event.repository.full_name.clone(),
                        r#ref: event.ref_.clone(),
                        after: event.after.clone(),
                    };

                    let executor = LocalJobExecutor;
                    // Create git provider for loading config
                    let provider_config = build_git_provider_config(&state.config)?;
                    let provider = create_git_provider(provider_config);

                    let yaml = codeza_cicd_engine::load_yaml_pipeline_config(
                        provider.as_ref(),
                        &event.repository.full_name,
                        &event.ref_,
                    )
                    .await;
                    let trigger_result = if let Some(yaml) = yaml {
                        trigger_push_pipeline_from_yaml(&executor, &ctx, &yaml).await
                    } else {
                        trigger_push_pipeline(&executor, &ctx).await
                    };
                    let pipeline = trigger_result.pipeline;

                    // Count CI pipelines triggered from Gitea push
                    state
                        .metrics
                        .register_counter("ci_pipelines_triggered_total.gitea".to_string())
                        .inc();

                    tracing::info!(
                        target = "cicd-trigger",
                        pipeline_id = %pipeline.id,
                        repo = %event.repository.full_name,
                        r#ref = %event.ref_,
                        "Created CI/CD pipeline stub from Gitea push",
                    );

                    if let Err(e) = repo
                        .create_execution(
                            "gitea",
                            &event.repository.full_name,
                            &event.ref_,
                            &event.after,
                            pipeline.id,
                        )
                        .await
                    {
                        tracing::warn!("Failed to persist pipeline execution: {}", e);
                    }

                    if let Some(execution) = trigger_result.job_execution {
                        tracing::info!(
                            target = "cicd-exec",
                            job_name = %execution.name,
                            status = ?execution.status,
                            duration = ?execution.duration,
                            "Executed CI/CD job from Gitea push",
                        );

                        if let Err(e) = repo
                            .create_job_execution(
                                pipeline.id,
                                "gitea",
                                &event.repository.full_name,
                                &event.ref_,
                                &event.after,
                                &execution,
                            )
                            .await
                        {
                            tracing::warn!("Failed to persist job execution: {}", e);
                        }
                    }
                }
            }
            codeza_git_service::WebhookEventType::PullRequest => {
                if let Ok(event) =
                    serde_json::from_slice::<codeza_git_service::PullRequestEvent>(&body)
                {
                    tracing::info!(
                        target = "git-webhook",
                        "Received Gitea pull request event #{} on repo {}",
                        event.pull_request.number,
                        event.repository.full_name,
                    );
                }
            }
            codeza_git_service::WebhookEventType::Issue => {
                if let Ok(event) = serde_json::from_slice::<codeza_git_service::IssueEvent>(&body) {
                    tracing::info!(
                        target = "git-webhook",
                        "Received Gitea issue event #{} on repo {}",
                        event.issue.number,
                        event.repository.full_name,
                    );
                }
            }
            _ => {
                tracing::info!(
                    target = "git-webhook",
                    "Received Gitea event {}",
                    event_header,
                );
            }
        }
    } else {
        tracing::info!(
            target = "git-webhook",
            "Received Gitea webhook with unknown event type: {}",
            event_header,
        );
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn handle_gitlab_webhook(
    state: AppState,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    let secret = state.config.git.webhook_secret.clone();
    let repo = PipelineExecutionRepository::new(state.pool.clone());

    let token = headers
        .get("X-Gitlab-Token")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            codeza_shared::CodezaError::AuthenticationError(
                "Missing X-Gitlab-Token header".to_string(),
            )
        })?;

    if token != secret {
        return Err(codeza_shared::CodezaError::AuthenticationError(
            "Invalid GitLab webhook token".to_string(),
        ));
    }

    let event_header = headers
        .get("X-Gitlab-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    #[derive(Debug, serde::Deserialize)]
    struct GitLabProject {
        path_with_namespace: String,
    }

    #[derive(Debug, serde::Deserialize)]
    struct GitLabPushEvent {
        #[serde(rename = "ref")]
        ref_: String,
        after: String,
        project: GitLabProject,
    }

    match event_header {
        "Push Hook" => {
            if let Ok(event) = serde_json::from_slice::<GitLabPushEvent>(&body) {
                tracing::info!(
                    target = "git-webhook",
                    "Received GitLab push event on repo {} (ref {}, after {})",
                    event.project.path_with_namespace,
                    event.ref_,
                    event.after,
                );

                let ctx = GitPushContext {
                    provider: "gitlab".to_string(),
                    repo: event.project.path_with_namespace.clone(),
                    r#ref: event.ref_.clone(),
                    after: event.after.clone(),
                };

                let executor = LocalJobExecutor;
                // Create git provider for loading config
                let provider_config = build_git_provider_config(&state.config)?;
                let provider = create_git_provider(provider_config);

                let yaml = codeza_cicd_engine::load_yaml_pipeline_config(
                    provider.as_ref(),
                    &event.project.path_with_namespace,
                    &event.ref_,
                )
                .await;
                let trigger_result = if let Some(yaml) = yaml {
                    trigger_push_pipeline_from_yaml(&executor, &ctx, &yaml).await
                } else {
                    trigger_push_pipeline(&executor, &ctx).await
                };
                let pipeline = trigger_result.pipeline;

                // Count CI pipelines triggered from GitLab push
                state
                    .metrics
                    .register_counter("ci_pipelines_triggered_total.gitlab".to_string())
                    .inc();

                tracing::info!(
                    target = "cicd-trigger",
                    pipeline_id = %pipeline.id,
                    repo = %event.project.path_with_namespace,
                    r#ref = %event.ref_,
                    "Created CI/CD pipeline stub from GitLab push",
                );

                if let Err(e) = repo
                    .create_execution(
                        "gitlab",
                        &event.project.path_with_namespace,
                        &event.ref_,
                        &event.after,
                        pipeline.id,
                    )
                    .await
                {
                    tracing::warn!("Failed to persist pipeline execution: {}", e);
                }

                if let Some(execution) = trigger_result.job_execution {
                    tracing::info!(
                        target = "cicd-exec",
                        job_name = %execution.name,
                        status = ?execution.status,
                        duration = ?execution.duration,
                        "Executed CI/CD job from GitLab push",
                    );

                    if let Err(e) = repo
                        .create_job_execution(
                            pipeline.id,
                            "gitlab",
                            &event.project.path_with_namespace,
                            &event.ref_,
                            &event.after,
                            &execution,
                        )
                        .await
                    {
                        tracing::warn!("Failed to persist job execution: {}", e);
                    }
                }
            } else {
                tracing::warn!(
                    target = "git-webhook",
                    "Failed to parse GitLab push payload for event: {}",
                    event_header,
                );
            }
        }
        _ => {
            tracing::info!(
                target = "git-webhook",
                "Received GitLab webhook event {}: {}",
                event_header,
                String::from_utf8_lossy(&body),
            );
        }
    }

    Ok(axum::http::StatusCode::NO_CONTENT)
}

use super::AppState;
use crate::routing::git::build_git_provider_config;
use sqlx::PgPool;
use codeza_cicd_engine::{LocalJobExecutor, GitPushContext, trigger_push_pipeline, trigger_push_pipeline_from_yaml};

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
    state
        .metrics
        .register_counter(counter_name)
        .inc();

    match provider_str.as_str() {
        "gitea" => handle_gitea_webhook(state.clone(), headers, body).await,
        "gitlab" => handle_gitlab_webhook(state.clone(), headers, body).await,
        other => Err(codeza_shared::CodezaError::ConfigError(format!(
            "Unsupported GIT_PROVIDER value for webhook: {}",
            other
        ))),
    }
}

async fn handle_gitea_webhook(
    state: AppState,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    let secret = state.config.git.webhook_secret.clone();
    let pool = state.pool.clone();

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
        state
            .metrics
            .register_counter(event_counter_name)
            .inc();

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
                    let yaml = load_yaml_pipeline_config(&state, &event.repository.full_name, &event.ref_).await;
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

                    persist_pipeline_execution(
                        &pool,
                        "gitea",
                        &event.repository.full_name,
                        &event.ref_,
                        &event.after,
                        pipeline.id,
                    )
                    .await;

                    if let Some(execution) = trigger_result.job_execution {
                        tracing::info!(
                            target = "cicd-exec",
                            job_name = %execution.name,
                            status = ?execution.status,
                            duration = ?execution.duration,
                            "Executed CI/CD job from Gitea push",
                        );

                        persist_job_execution(
                            &pool,
                            "gitea",
                            &event.repository.full_name,
                            &event.ref_,
                            &event.after,
                            pipeline.id,
                            &execution,
                        )
                        .await;
                    }
                }
            }
            codeza_git_service::WebhookEventType::PullRequest => {
                if let Ok(event) = serde_json::from_slice::<codeza_git_service::PullRequestEvent>(&body) {
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

async fn persist_pipeline_execution(
    pool: &PgPool,
    provider: &str,
    repo: &str,
    r#ref: &str,
    commit: &str,
    pipeline_id: uuid::Uuid,
) {
    let result = sqlx::query(
        "INSERT INTO ci_pipeline_executions \
         (id, provider, repo, git_ref, commit_sha, pipeline_id, created_at) \
         VALUES ($1, $2, $3, $4, $5, $6, NOW())",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(provider)
    .bind(repo)
    .bind(r#ref)
    .bind(commit)
    .bind(pipeline_id)
    .execute(pool)
    .await;

    if let Err(e) = result {
        tracing::warn!(
            target = "cicd-persist",
            error = %e,
            provider = provider,
            repo = repo,
            r#ref = r#ref,
            commit = commit,
            pipeline_id = %pipeline_id,
            "Failed to persist pipeline execution; continuing without error",
        );
    }
}

async fn persist_job_execution(
    pool: &PgPool,
    provider: &str,
    repo: &str,
    r#ref: &str,
    commit: &str,
    pipeline_id: uuid::Uuid,
    job: &codeza_cicd_engine::pipeline::JobExecution,
) {
    let duration = job.duration.map(|d| d as i64);

    let result = sqlx::query(
        "INSERT INTO ci_job_executions \
         (id, pipeline_id, provider, repo, git_ref, commit_sha, job_id, job_name, status, started_at, finished_at, duration_seconds, log_url, created_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW())",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(pipeline_id)
    .bind(provider)
    .bind(repo)
    .bind(r#ref)
    .bind(commit)
    .bind(job.id)
    .bind(&job.name)
    .bind(format!("{:?}", job.status))
    .bind(job.started_at)
    .bind(job.finished_at)
    .bind(duration)
    .bind(&job.log_url)
    .execute(pool)
    .await;

    if let Err(e) = result {
        tracing::warn!(
            target = "cicd-persist",
            error = %e,
            provider = provider,
            repo = repo,
            r#ref = r#ref,
            commit = commit,
            pipeline_id = %pipeline_id,
            job_id = %job.id,
            job_name = %job.name,
            "Failed to persist job execution; continuing without error",
        );
    }
}

async fn handle_gitlab_webhook(
    state: AppState,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    let secret = state.config.git.webhook_secret.clone();
    let pool = state.pool.clone();

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
                let yaml = load_yaml_pipeline_config(&state, &event.project.path_with_namespace, &event.ref_).await;
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
                persist_pipeline_execution(
                    &pool,
                    "gitlab",
                    &event.project.path_with_namespace,
                    &event.ref_,
                    &event.after,
                    pipeline.id,
                )
                .await;

                if let Some(execution) = trigger_result.job_execution {
                    tracing::info!(
                        target = "cicd-exec",
                        job_name = %execution.name,
                        status = ?execution.status,
                        duration = ?execution.duration,
                        "Executed CI/CD job from GitLab push",
                    );

                    persist_job_execution(
                        &pool,
                        "gitlab",
                        &event.project.path_with_namespace,
                        &event.ref_,
                        &event.after,
                        pipeline.id,
                        &execution,
                    )
                    .await;
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

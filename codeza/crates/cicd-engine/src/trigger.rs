//! Helpers for triggering CI/CD pipelines from external events (e.g. Git push)

use crate::executor::JobExecutor;
use crate::pipeline::{Job, JobExecution, Pipeline, Stage};
use std::collections::HashMap;
use uuid::Uuid;

/// Normalized context for a Git push event
#[derive(Debug, Clone)]
pub struct GitPushContext {
    pub provider: String,
    pub repo: String,
    pub r#ref: String,
    pub after: String,
}

/// Result of triggering a pipeline from a Git push
#[derive(Debug, Clone)]
pub struct TriggerResult {
    pub pipeline: Pipeline,
    pub job_execution: Option<JobExecution>,
}

/// Build a simple pipeline stub for a Git push and execute its single job using the given executor.
///
/// This helper is intentionally best-effort: job execution failures are logged and surfaced as
/// `job_execution = None`, but the pipeline object is still returned so callers can persist it.
pub async fn trigger_push_pipeline(
    executor: &dyn JobExecutor,
    ctx: &GitPushContext,
) -> TriggerResult {
    let span = tracing::span!(
        tracing::Level::INFO,
        "ci_trigger_push",
        provider = %ctx.provider,
        repo = %ctx.repo,
        r#ref = %ctx.r#ref,
    );
    let _enter = span.enter();

    let pipeline_name = format!("[push] {} {}", ctx.repo, ctx.after);

    let mut variables: HashMap<String, String> = HashMap::new();
    variables.insert("CI_PROVIDER".to_string(), ctx.provider.clone());
    variables.insert("CI_REPO".to_string(), ctx.repo.clone());
    variables.insert("CI_REF".to_string(), ctx.r#ref.clone());
    variables.insert("CI_COMMIT_SHA".to_string(), ctx.after.clone());

    let job = Job {
        id: Uuid::new_v4(),
        name: "default".to_string(),
        description: Some(format!(
            "Auto-generated pipeline from {} push",
            ctx.provider
        )),
        image: "alpine:latest".to_string(),
        script: vec![format!(
            "echo 'Codeza CI pipeline triggered from {} push on {} ({})'",
            ctx.provider, ctx.repo, ctx.r#ref
        )],
        before_script: None,
        after_script: None,
        artifacts: None,
        cache: None,
        variables: None,
        timeout: None,
        retry: None,
        allow_failure: None,
    };

    let stage = Stage {
        name: "build".to_string(),
        description: Some("Default build stage".to_string()),
        jobs: vec![job.clone()],
        when: None,
    };

    let pipeline = Pipeline::new(pipeline_name, vec![stage]);

    let job_execution = match executor.execute(&job, &variables).await {
        Ok(execution) => Some(execution),
        Err(err) => {
            tracing::warn!(
                target = "cicd-exec",
                error = %err,
                provider = %ctx.provider,
                repo = %ctx.repo,
                r#ref = %ctx.r#ref,
                "Failed to execute CI/CD job for Git push pipeline",
            );
            None
        }
    };

    TriggerResult {
        pipeline,
        job_execution,
    }
}

pub fn pipeline_from_yaml_str(yaml: &str) -> anyhow::Result<Pipeline> {
    #[derive(Debug, serde::Deserialize)]
    struct YamlJob {
        name: String,
        #[serde(default)]
        description: Option<String>,
        image: String,
        script: Vec<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct YamlStage {
        name: String,
        #[serde(default)]
        description: Option<String>,
        jobs: Vec<YamlJob>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct YamlPipeline {
        name: String,
        stages: Vec<YamlStage>,
    }

    let parsed: YamlPipeline = serde_yaml::from_str(yaml)?;

    let stages: Vec<Stage> = parsed
        .stages
        .into_iter()
        .map(|s| Stage {
            name: s.name,
            description: s.description,
            jobs: s
                .jobs
                .into_iter()
                .map(|j| Job {
                    id: Uuid::new_v4(),
                    name: j.name,
                    description: j.description,
                    image: j.image,
                    script: j.script,
                    before_script: None,
                    after_script: None,
                    artifacts: None,
                    cache: None,
                    variables: None,
                    timeout: None,
                    retry: None,
                    allow_failure: None,
                })
                .collect(),
            when: None,
        })
        .collect();

    Ok(Pipeline::new(parsed.name, stages))
}

pub async fn trigger_push_pipeline_from_yaml(
    executor: &dyn JobExecutor,
    ctx: &GitPushContext,
    yaml: &str,
) -> TriggerResult {
    let span = tracing::span!(
        tracing::Level::INFO,
        "ci_trigger_push_yaml",
        provider = %ctx.provider,
        repo = %ctx.repo,
        r#ref = %ctx.r#ref,
    );
    let _enter = span.enter();

    let pipeline = match pipeline_from_yaml_str(yaml) {
        Ok(p) => p,
        Err(err) => {
            tracing::warn!(
                target = "cicd-trigger",
                error = %err,
                provider = %ctx.provider,
                repo = %ctx.repo,
                r#ref = %ctx.r#ref,
                "Failed to parse YAML pipeline; falling back to default stub",
            );
            return trigger_push_pipeline(executor, ctx).await;
        }
    };

    let first_job = pipeline
        .stages
        .first()
        .and_then(|s| s.jobs.first())
        .cloned();

    let job = match first_job {
        Some(j) => j,
        None => {
            tracing::warn!(
                target = "cicd-trigger",
                provider = %ctx.provider,
                repo = %ctx.repo,
                r#ref = %ctx.r#ref,
                "YAML pipeline has no stages/jobs; falling back to default stub",
            );
            return trigger_push_pipeline(executor, ctx).await;
        }
    };

    let mut variables: HashMap<String, String> = HashMap::new();
    variables.insert("CI_PROVIDER".to_string(), ctx.provider.clone());
    variables.insert("CI_REPO".to_string(), ctx.repo.clone());
    variables.insert("CI_REF".to_string(), ctx.r#ref.clone());
    variables.insert("CI_COMMIT_SHA".to_string(), ctx.after.clone());

    let job_execution = match executor.execute(&job, &variables).await {
        Ok(execution) => Some(execution),
        Err(err) => {
            tracing::warn!(
                target = "cicd-exec",
                error = %err,
                provider = %ctx.provider,
                repo = %ctx.repo,
                r#ref = %ctx.r#ref,
                "Failed to execute CI/CD job for YAML pipeline; falling back to stub next time",
            );
            None
        }
    };

    TriggerResult {
        pipeline,
        job_execution,
    }
}

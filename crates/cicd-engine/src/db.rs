use chrono::{DateTime, Utc};
use codeza_shared::CodezaError;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PipelineExecutionRecord {
    pub id: Uuid,
    pub provider: String,
    pub repo: String,
    pub git_ref: String,
    pub commit_sha: String,
    pub pipeline_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JobExecutionRecord {
    pub id: Uuid,
    pub pipeline_id: Uuid,
    pub provider: String,
    pub repo: String,
    pub git_ref: String,
    pub commit_sha: String,
    pub job_id: Uuid,
    pub job_name: String,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub log_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct PipelineExecutionRepository {
    pool: PgPool,
}

impl PipelineExecutionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_execution(
        &self,
        provider: &str,
        repo: &str,
        git_ref: &str,
        commit: &str,
        pipeline_id: Uuid,
    ) -> Result<(), CodezaError> {
        sqlx::query(
            "INSERT INTO ci_pipeline_executions \
             (id, provider, repo, git_ref, commit_sha, pipeline_id, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
        )
        .bind(Uuid::new_v4())
        .bind(provider)
        .bind(repo)
        .bind(git_ref)
        .bind(commit)
        .bind(pipeline_id)
        .execute(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn create_job_execution(
        &self,
        pipeline_id: Uuid,
        provider: &str,
        repo: &str,
        git_ref: &str,
        commit: &str,
        job: &crate::pipeline::JobExecution,
    ) -> Result<(), CodezaError> {
        let duration = job.duration.map(|d| d as i64);

        sqlx::query(
            "INSERT INTO ci_job_executions \
             (id, pipeline_id, provider, repo, git_ref, commit_sha, job_id, job_name, status, started_at, finished_at, duration_seconds, log_url, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW())",
        )
        .bind(Uuid::new_v4())
        .bind(pipeline_id)
        .bind(provider)
        .bind(repo)
        .bind(git_ref)
        .bind(commit)
        .bind(job.id)
        .bind(&job.name)
        .bind(format!("{:?}", job.status))
        .bind(job.started_at)
        .bind(job.finished_at)
        .bind(duration)
        .bind(&job.log_url)
        .execute(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn list_executions(&self) -> Result<Vec<PipelineExecutionRecord>, CodezaError> {
        let rows = sqlx::query(
            "SELECT id, provider, repo, git_ref, commit_sha, pipeline_id, created_at \
             FROM ci_pipeline_executions \
             ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|row| PipelineExecutionRecord {
                id: row.get("id"),
                provider: row.get("provider"),
                repo: row.get("repo"),
                git_ref: row.get("git_ref"),
                commit_sha: row.get("commit_sha"),
                pipeline_id: row.get("pipeline_id"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(items)
    }

    pub async fn get_execution(&self, id: Uuid) -> Result<PipelineExecutionRecord, CodezaError> {
        let row = sqlx::query(
            "SELECT id, provider, repo, git_ref, commit_sha, pipeline_id, created_at \
             FROM ci_pipeline_executions \
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => Ok(PipelineExecutionRecord {
                id: row.get("id"),
                provider: row.get("provider"),
                repo: row.get("repo"),
                git_ref: row.get("git_ref"),
                commit_sha: row.get("commit_sha"),
                pipeline_id: row.get("pipeline_id"),
                created_at: row.get("created_at"),
            }),
            None => Err(CodezaError::NotFound(format!(
                "Pipeline execution {} not found",
                id
            ))),
        }
    }

    pub async fn list_job_executions(
        &self,
        pipeline_id: Uuid,
    ) -> Result<Vec<JobExecutionRecord>, CodezaError> {
        let rows = sqlx::query(
            "SELECT id, pipeline_id, provider, repo, git_ref, commit_sha, job_id, job_name, status, \
                     started_at, finished_at, duration_seconds, log_url, created_at \
             FROM ci_job_executions \
             WHERE pipeline_id = $1 \
             ORDER BY created_at DESC",
        )
        .bind(pipeline_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|row| JobExecutionRecord {
                id: row.get("id"),
                pipeline_id: row.get("pipeline_id"),
                provider: row.get("provider"),
                repo: row.get("repo"),
                git_ref: row.get("git_ref"),
                commit_sha: row.get("commit_sha"),
                job_id: row.get("job_id"),
                job_name: row.get("job_name"),
                status: row.get("status"),
                started_at: row.get("started_at"),
                finished_at: row.get("finished_at"),
                duration_seconds: row.get("duration_seconds"),
                log_url: row.get("log_url"),
                created_at: row.get("created_at"),
            })
            .collect();

        Ok(items)
    }
}

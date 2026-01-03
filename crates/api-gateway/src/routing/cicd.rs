use super::AppState;
use serde::Serialize;
use sqlx::Row;

#[derive(Debug, Serialize)]
struct PipelineExecutionRecord {
    id: uuid::Uuid,
    provider: String,
    repo: String,
    git_ref: String,
    commit_sha: String,
    pipeline_id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
struct JobExecutionRecord {
    id: uuid::Uuid,
    pipeline_id: uuid::Uuid,
    provider: String,
    repo: String,
    git_ref: String,
    commit_sha: String,
    job_id: uuid::Uuid,
    job_name: String,
    status: String,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    finished_at: Option<chrono::DateTime<chrono::Utc>>,
    duration_seconds: Option<i64>,
    log_url: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_pipelines(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<axum::Json<Vec<PipelineExecutionRecord>>, codeza_shared::CodezaError> {
    let rows = sqlx::query(
        "SELECT id, provider, repo, git_ref, commit_sha, pipeline_id, created_at \
         FROM ci_pipeline_executions \
         ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| codeza_shared::CodezaError::DatabaseError(e.to_string()))?;

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

    Ok(axum::Json(items))
}

pub async fn get_pipeline_execution(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<PipelineExecutionRecord>, codeza_shared::CodezaError> {
    let row = sqlx::query(
        "SELECT id, provider, repo, git_ref, commit_sha, pipeline_id, created_at \
         FROM ci_pipeline_executions \
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| codeza_shared::CodezaError::DatabaseError(e.to_string()))?;

    let row = match row {
        Some(row) => row,
        None => {
            return Err(codeza_shared::CodezaError::NotFound(format!(
                "Pipeline execution {} not found",
                id
            )))
        }
    };

    let record = PipelineExecutionRecord {
        id: row.get("id"),
        provider: row.get("provider"),
        repo: row.get("repo"),
        git_ref: row.get("git_ref"),
        commit_sha: row.get("commit_sha"),
        pipeline_id: row.get("pipeline_id"),
        created_at: row.get("created_at"),
    };

    Ok(axum::Json(record))
}

pub async fn list_pipeline_jobs(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pipeline_id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<Vec<JobExecutionRecord>>, codeza_shared::CodezaError> {
    let rows = sqlx::query(
        "SELECT id, pipeline_id, provider, repo, git_ref, commit_sha, job_id, job_name, status, \
                 started_at, finished_at, duration_seconds, log_url, created_at \
         FROM ci_job_executions \
         WHERE pipeline_id = $1 \
         ORDER BY created_at DESC",
    )
    .bind(pipeline_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| codeza_shared::CodezaError::DatabaseError(e.to_string()))?;

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

    Ok(axum::Json(items))
}

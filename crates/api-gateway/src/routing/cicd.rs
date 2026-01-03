use super::AppState;
use uuid::Uuid;
use codeza_cicd_engine::{JobExecutionRecord, PipelineExecutionRecord, PipelineExecutionRepository};

#[utoipa::path(
    get,
    path = "/api/v1/pipelines",
    responses(
        (status = 200, description = "List of pipeline executions", body = Vec<PipelineExecutionRecord>),
        (status = 500, description = "Internal server error")
    ),
    tag = "cicd"
)]
pub async fn list_pipelines(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<axum::Json<Vec<PipelineExecutionRecord>>, codeza_shared::CodezaError> {
    let repo = PipelineExecutionRepository::new(state.pool);
    let items = repo.list_executions().await?;
    Ok(axum::Json(items))
}

#[utoipa::path(
    get,
    path = "/api/v1/pipelines/{id}",
    params(
        ("id" = Uuid, Path, description = "Pipeline execution ID")
    ),
    responses(
        (status = 200, description = "Pipeline execution details", body = PipelineExecutionRecord),
        (status = 404, description = "Execution not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "cicd"
)]
pub async fn get_pipeline_execution(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<PipelineExecutionRecord>, codeza_shared::CodezaError> {
    let repo = PipelineExecutionRepository::new(state.pool);
    let item = repo.get_execution(id).await?;
    Ok(axum::Json(item))
}

#[utoipa::path(
    get,
    path = "/api/v1/pipelines/{id}/jobs",
    params(
        ("id" = Uuid, Path, description = "Pipeline execution ID")
    ),
    responses(
        (status = 200, description = "List of jobs for pipeline", body = Vec<JobExecutionRecord>),
        (status = 500, description = "Internal server error")
    ),
    tag = "cicd"
)]
pub async fn list_pipeline_jobs(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(pipeline_id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<Vec<JobExecutionRecord>>, codeza_shared::CodezaError> {
    let repo = PipelineExecutionRepository::new(state.pool);
    let items = repo.list_job_executions(pipeline_id).await?;
    Ok(axum::Json(items))
}

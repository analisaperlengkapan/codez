use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{ActionWorkflow, CreateWorkflowRunOption, WorkflowRun, Activity};
use crate::router::AppState;

pub async fn list_workflows(State(_state): State<AppState>, Path((_owner, _repo)): Path<(String, String)>) -> Json<Vec<ActionWorkflow>> {
    // In a real implementation, we would scan .github/workflows or .gitea/workflows
    // Mocking for now as existing code does
    let wfs = vec![
        ActionWorkflow { id: 1, name: "CI".to_string(), status: "active".to_string() }
    ];
    Json(wfs)
}

pub async fn trigger_workflow(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(_payload): Json<CreateWorkflowRunOption>
) -> (StatusCode, Json<WorkflowRun>) {
    // Verify workflow exists (mock check)
    if id != 1 {
        return (StatusCode::NOT_FOUND, Json(WorkflowRun { id: 0, workflow_id: 0, status: "".to_string(), created_at: "".to_string() }));
    }

    let mut runs = state.workflow_runs.write().unwrap();
    let run_id = (runs.len() as u64) + 1;
    let run = WorkflowRun {
        id: run_id,
        workflow_id: id,
        status: "queued".to_string(),
        created_at: "now".to_string(),
    };
    runs.push(run.clone());

    // Log Activity
    let mut activities = state.activities.write().unwrap();
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id: 0, // Should look up repo ID
        user_id: 1, // mock admin
        user_name: "admin".to_string(),
        op_type: "trigger_workflow".to_string(),
        content: format!("triggered workflow run #{} on {}/{}", run_id, owner, repo_name),
        created: "now".to_string(),
    });

    (StatusCode::CREATED, Json(run))
}

pub async fn list_workflow_runs(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>
) -> Json<Vec<WorkflowRun>> {
    let runs = state.workflow_runs.read().unwrap();
    let filtered: Vec<WorkflowRun> = runs.iter().filter(|r| r.workflow_id == id).cloned().collect();
    Json(filtered)
}

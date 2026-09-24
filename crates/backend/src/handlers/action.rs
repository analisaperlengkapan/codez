use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{
    ActionWorkflow, Activity, CreateWorkflowRunOption, UpdateWorkflowRunOption, WorkflowRun,
    WorkflowStepLog,
};

pub async fn list_workflows(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<ActionWorkflow>> {
    let repo_id = {
        let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
        repos
            .iter()
            .find(|r| r.owner == owner && r.name == repo_name)
            .map(|r| r.id)
            .unwrap_or(0)
    };
    let wfs = state.workflows.read().unwrap_or_else(|e| e.into_inner());
    let filtered: Vec<ActionWorkflow> = wfs
        .iter()
        .filter(|w| w.repo_id == repo_id)
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn trigger_workflow(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(_payload): Json<CreateWorkflowRunOption>,
) -> (StatusCode, Json<WorkflowRun>) {
    let repo_id = {
        let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
        repos
            .iter()
            .find(|r| r.owner == owner && r.name == repo_name)
            .map(|r| r.id)
            .unwrap_or(0)
    };

    // Verify workflow exists
    let exists = {
        let wfs = state.workflows.read().unwrap_or_else(|e| e.into_inner());
        wfs.iter().any(|w| w.id == id && w.repo_id == repo_id)
    };

    if !exists {
        return (
            StatusCode::NOT_FOUND,
            Json(WorkflowRun {
                id: 0,
                workflow_id: 0,
                status: "".to_string(),
                created_at: "".to_string(),
                step_logs: vec![],
            }),
        );
    }

    let mut runs = state
        .workflow_runs
        .write()
        .unwrap_or_else(|e| e.into_inner());
    let run_id = runs.iter().map(|r| r.id).max().unwrap_or(0) + 1;
    let run = WorkflowRun {
        id: run_id,
        workflow_id: id,
        status: "queued".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        step_logs: vec![],
    };
    runs.push(run.clone());

    // Log Activity
    let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id,
        user_id: 1, // mock admin
        user_name: "admin".to_string(),
        op_type: "trigger_workflow".to_string(),
        content: format!(
            "triggered workflow run #{} on {}/{}",
            run_id, owner, repo_name
        ),
        created: "now".to_string(),
    });

    (StatusCode::CREATED, Json(run))
}

pub async fn list_workflow_runs(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
) -> Json<Vec<WorkflowRun>> {
    let repo_id = {
        let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
        repos
            .iter()
            .find(|r| r.owner == owner && r.name == repo_name)
            .map(|r| r.id)
            .unwrap_or(0)
    };

    let valid_workflow = {
        let wfs = state.workflows.read().unwrap_or_else(|e| e.into_inner());
        wfs.iter().any(|w| w.id == id && w.repo_id == repo_id)
    };

    if !valid_workflow {
        return Json(vec![]);
    }

    let runs = state
        .workflow_runs
        .read()
        .unwrap_or_else(|e| e.into_inner());
    let filtered: Vec<WorkflowRun> = runs
        .iter()
        .filter(|r| r.workflow_id == id)
        .cloned()
        .collect();
    Json(filtered)
}

pub async fn get_workflow_run_logs(
    State(state): State<AppState>,
    Path((_owner, _repo, run_id)): Path<(String, String, u64)>,
) -> (StatusCode, Json<Vec<WorkflowStepLog>>) {
    let runs = state
        .workflow_runs
        .read()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(run) = runs.iter().find(|r| r.id == run_id) {
        (StatusCode::OK, Json(run.step_logs.clone()))
    } else {
        (StatusCode::NOT_FOUND, Json(vec![]))
    }
}

pub async fn rerun_workflow_run(
    State(state): State<AppState>,
    Path((owner, repo_name, run_id)): Path<(String, String, u64)>,
) -> (StatusCode, Json<WorkflowRun>) {
    let repo_id = {
        let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
        repos
            .iter()
            .find(|r| r.owner == owner && r.name == repo_name)
            .map(|r| r.id)
            .unwrap_or(0)
    };

    let mut runs = state
        .workflow_runs
        .write()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(run) = runs.iter_mut().find(|r| r.id == run_id) {
        run.status = "success".to_string();
        run.created_at = chrono::Utc::now().to_rfc3339();
        run.step_logs = vec![
            WorkflowStepLog {
                name: "Set up job (Re-run)".to_string(),
                status: "success".to_string(),
                logs: vec!["Re-running workflow steps...".to_string()],
            },
            WorkflowStepLog {
                name: "Execute build & test".to_string(),
                status: "success".to_string(),
                logs: vec!["Cargo test completed without errors.".to_string()],
            },
        ];

        let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
        let activity_id = (activities.len() as u64) + 1;
        activities.push(Activity {
            id: activity_id,
            repo_id,
            user_id: 1,
            user_name: "admin".to_string(),
            op_type: "rerun_workflow".to_string(),
            content: format!("re-ran workflow run #{} on {}/{}", run_id, owner, repo_name),
            created: "now".to_string(),
        });

        (StatusCode::OK, Json(run.clone()))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(WorkflowRun {
                id: 0,
                workflow_id: 0,
                status: "".to_string(),
                created_at: "".to_string(),
                step_logs: vec![],
            }),
        )
    }
}

pub async fn update_workflow_run(
    State(state): State<AppState>,
    Path((owner, repo_name, run_id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateWorkflowRunOption>,
) -> (StatusCode, Json<WorkflowRun>) {
    let repo_id = {
        let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
        repos
            .iter()
            .find(|r| r.owner == owner && r.name == repo_name)
            .map(|r| r.id)
            .unwrap_or(0)
    };

    let workflow_id = {
        let runs = state
            .workflow_runs
            .read()
            .unwrap_or_else(|e| e.into_inner());
        match runs.iter().find(|r| r.id == run_id) {
            Some(run) => run.workflow_id,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(WorkflowRun {
                        id: 0,
                        workflow_id: 0,
                        status: "".to_string(),
                        created_at: "".to_string(),
                        step_logs: vec![],
                    }),
                )
            }
        }
    };

    let valid_workflow = {
        let wfs = state.workflows.read().unwrap_or_else(|e| e.into_inner());
        wfs.iter()
            .any(|w| w.id == workflow_id && w.repo_id == repo_id)
    };

    if !valid_workflow {
        return (
            StatusCode::NOT_FOUND,
            Json(WorkflowRun {
                id: 0,
                workflow_id: 0,
                status: "".to_string(),
                created_at: "".to_string(),
                step_logs: vec![],
            }),
        );
    }

    let mut runs = state
        .workflow_runs
        .write()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(run) = runs.iter_mut().find(|r| r.id == run_id) {
        run.status = payload.status.clone();

        let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
        let activity_id = (activities.len() as u64) + 1;
        activities.push(Activity {
            id: activity_id,
            repo_id,
            user_id: 1, // Mock
            user_name: "admin".to_string(),
            op_type: "update_workflow_run".to_string(),
            content: format!(
                "updated workflow run #{} on {}/{} to {}",
                run_id, owner, repo_name, payload.status
            ),
            created: "now".to_string(),
        });

        (StatusCode::OK, Json(run.clone()))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(WorkflowRun {
                id: 0,
                workflow_id: 0,
                status: "".to_string(),
                created_at: "".to_string(),
                step_logs: vec![],
            }),
        )
    }
}

pub async fn delete_workflow_run(
    State(state): State<AppState>,
    Path((owner, repo_name, run_id)): Path<(String, String, u64)>,
) -> StatusCode {
    let repo_id = {
        let repos = state.repos.read().unwrap_or_else(|e| e.into_inner());
        repos
            .iter()
            .find(|r| r.owner == owner && r.name == repo_name)
            .map(|r| r.id)
            .unwrap_or(0)
    };

    let workflow_id = {
        let runs = state
            .workflow_runs
            .read()
            .unwrap_or_else(|e| e.into_inner());
        match runs.iter().find(|r| r.id == run_id) {
            Some(run) => run.workflow_id,
            None => return StatusCode::NOT_FOUND,
        }
    };

    let valid_workflow = {
        let wfs = state.workflows.read().unwrap_or_else(|e| e.into_inner());
        wfs.iter()
            .any(|w| w.id == workflow_id && w.repo_id == repo_id)
    };

    if !valid_workflow {
        return StatusCode::NOT_FOUND;
    }

    let mut runs = state
        .workflow_runs
        .write()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(pos) = runs.iter().position(|r| r.id == run_id) {
        runs.remove(pos);

        let mut activities = state.activities.write().unwrap_or_else(|e| e.into_inner());
        let activity_id = (activities.len() as u64) + 1;
        activities.push(Activity {
            id: activity_id,
            repo_id,
            user_id: 1, // Mock
            user_name: "admin".to_string(),
            op_type: "delete_workflow_run".to_string(),
            content: format!(
                "deleted workflow run #{} on {}/{}",
                run_id, owner, repo_name
            ),
            created: "now".to_string(),
        });

        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

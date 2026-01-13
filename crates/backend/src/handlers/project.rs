use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{
    Project, CreateProjectOption, ProjectColumn, CreateProjectColumnOption,
    ProjectCard, CreateProjectCardOption, MoveProjectCardOption
};
use crate::router::AppState;

pub async fn list_projects(State(state): State<AppState>, Path((owner, repo_name)): Path<(String, String)>) -> Json<Vec<Project>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos.iter().find(|r| r.owner == owner && r.name == repo_name).map(|r| r.id).unwrap_or(0);

    let projects = state.projects.read().unwrap();
    let filtered: Vec<Project> = projects.iter().filter(|p| p.repo_id == repo_id).cloned().collect();
    Json(filtered)
}

pub async fn create_project(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateProjectOption>
) -> (StatusCode, Json<Project>) {
    let repos = state.repos.read().unwrap();
    let repo = repos.iter().find(|r| r.owner == owner && r.name == repo_name);

    if repo.is_none() {
         return (StatusCode::NOT_FOUND, Json(Project {
            id: 0, repo_id: 0, title: "".to_string(), description: None, is_closed: false
        }));
    }
    let repo_id = repo.unwrap().id;

    let mut projects = state.projects.write().unwrap();
    let id = (projects.len() as u64) + 1;
    let project = Project {
        id,
        repo_id,
        title: payload.title,
        description: payload.description,
        is_closed: false,
    };
    projects.push(project.clone());
    (StatusCode::CREATED, Json(project))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path((_owner, _repo, id)): Path<(String, String, u64)>
) -> Json<Option<Project>> {
    let projects = state.projects.read().unwrap();
    let project = projects.iter().find(|p| p.id == id).cloned();
    Json(project)
}

pub async fn list_project_columns(
    State(state): State<AppState>,
    Path((_owner, _repo, project_id)): Path<(String, String, u64)>
) -> Json<Vec<ProjectColumn>> {
    let columns = state.project_columns.read().unwrap();
    let mut filtered: Vec<ProjectColumn> = columns.iter()
        .filter(|c| c.project_id == project_id)
        .cloned()
        .collect();
    filtered.sort_by_key(|c| c.ordering);
    Json(filtered)
}

pub async fn create_project_column(
    State(state): State<AppState>,
    Path((_owner, _repo, project_id)): Path<(String, String, u64)>,
    Json(payload): Json<CreateProjectColumnOption>
) -> (StatusCode, Json<ProjectColumn>) {
    let mut columns = state.project_columns.write().unwrap();
    let id = (columns.len() as u64) + 1;
    let count = columns.iter().filter(|c| c.project_id == project_id).count() as u64;

    let column = ProjectColumn {
        id,
        project_id,
        title: payload.title,
        ordering: count,
    };
    columns.push(column.clone());
    (StatusCode::CREATED, Json(column))
}

pub async fn list_project_cards(
    State(state): State<AppState>,
    Path((_owner, _repo, column_id)): Path<(String, String, u64)>
) -> Json<Vec<ProjectCard>> {
    let cards = state.project_cards.read().unwrap();
    let mut filtered: Vec<ProjectCard> = cards.iter()
        .filter(|c| c.column_id == column_id)
        .cloned()
        .collect();
    filtered.sort_by_key(|c| c.ordering);
    Json(filtered)
}

pub async fn create_project_card(
    State(state): State<AppState>,
    Path((_owner, _repo, column_id)): Path<(String, String, u64)>,
    Json(payload): Json<CreateProjectCardOption>
) -> (StatusCode, Json<ProjectCard>) {
    let mut cards = state.project_cards.write().unwrap();
    let id = (cards.len() as u64) + 1;
    let count = cards.iter().filter(|c| c.column_id == column_id).count() as u64;

    let card = ProjectCard {
        id,
        column_id,
        content: payload.content,
        note: payload.note,
        issue_id: payload.issue_id,
        ordering: count,
    };
    cards.push(card.clone());
    (StatusCode::CREATED, Json(card))
}

pub async fn move_project_card(
    State(state): State<AppState>,
    Path((_owner, _repo, card_id)): Path<(String, String, u64)>,
    Json(payload): Json<MoveProjectCardOption>
) -> StatusCode {
    let mut cards = state.project_cards.write().unwrap();
    if let Some(card) = cards.iter_mut().find(|c| c.id == card_id) {
        card.column_id = payload.column_id;
        // In real impl, we would handle re-ordering other cards in the lists.
        // For mock, just updating column is sufficient to show movement.
        card.ordering = payload.new_index;
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

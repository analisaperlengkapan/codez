use crate::router::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{CreateReleaseOption, Release, ReleaseAsset, UpdateReleaseOption, User};

pub async fn list_releases(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Json<Vec<Release>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let releases = state.releases.read().unwrap();
    let mut filtered_releases: Vec<Release> = releases
        .iter()
        .filter(|r| r.repo_id == repo_id)
        .cloned()
        .collect();
    // Sort by ID descending (newest first)
    filtered_releases.sort_by(|a, b| b.id.cmp(&a.id));
    Json(filtered_releases)
}

pub async fn get_release(
    State(state): State<AppState>,
    Path((owner, repo_name, id_or_tag)): Path<(String, String, String)>,
) -> Json<Option<Release>> {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let releases = state.releases.read().unwrap();

    if id_or_tag == "latest" {
        // Find latest release (not draft, not prerelease)
        let release = releases
            .iter()
            .filter(|r| r.repo_id == repo_id && !r.draft && !r.prerelease)
            .max_by_key(|r| r.id)
            .cloned();
        return Json(release);
    }

    // Check if input is ID or Tag
    let release = if let Ok(id) = id_or_tag.parse::<u64>() {
        releases
            .iter()
            .find(|r| r.repo_id == repo_id && r.id == id)
            .cloned()
    } else {
        releases
            .iter()
            .find(|r| r.repo_id == repo_id && r.tag_name == id_or_tag)
            .cloned()
    };

    Json(release)
}

pub async fn create_release(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
    Json(payload): Json<CreateReleaseOption>,
) -> (StatusCode, Json<Release>) {
    let repos = state.repos.read().unwrap();
    let repo = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name);

    let repo_id = if let Some(r) = repo {
        r.id
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Release {
                id: 0,
                repo_id: 0,
                tag_name: "".to_string(),
                name: "".to_string(),
                body: None,
                draft: false,
                prerelease: false,
                created_at: "".to_string(),
                author: User::new(0, "".to_string(), None),
                assets: vec![],
            }),
        );
    };

    let mut releases = state.releases.write().unwrap();
    // Generate safe ID
    let id = releases.iter().map(|r| r.id).max().unwrap_or(0) + 1;

    let release = Release {
        id,
        repo_id,
        tag_name: payload.tag_name,
        name: payload.name,
        body: payload.body,
        draft: payload.draft,
        prerelease: payload.prerelease,
        created_at: "now".to_string(),
        author: User::new(1, "admin".to_string(), None),
        assets: vec![],
    };
    releases.push(release.clone());
    (StatusCode::CREATED, Json(release))
}

pub async fn update_release(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
    Json(payload): Json<UpdateReleaseOption>,
) -> (StatusCode, Json<Option<Release>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let mut releases = state.releases.write().unwrap();
    let release = releases
        .iter_mut()
        .find(|r| r.repo_id == repo_id && r.id == id);

    if let Some(r) = release {
        if let Some(tag_name) = payload.tag_name {
            r.tag_name = tag_name;
        }
        if let Some(name) = payload.name {
            r.name = name;
        }
        if let Some(body) = payload.body {
            r.body = Some(body);
        }
        if let Some(draft) = payload.draft {
            r.draft = draft;
        }
        if let Some(prerelease) = payload.prerelease {
            r.prerelease = prerelease;
        }

        return (StatusCode::OK, Json(Some(r.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_release(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
) -> StatusCode {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let mut releases = state.releases.write().unwrap();
    if let Some(pos) = releases
        .iter()
        .position(|r| r.repo_id == repo_id && r.id == id)
    {
        releases.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn upload_release_asset(
    State(state): State<AppState>,
    Path((owner, repo_name, id)): Path<(String, String, u64)>,
) -> (StatusCode, Json<Option<ReleaseAsset>>) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let mut releases = state.releases.write().unwrap();
    let release = releases
        .iter_mut()
        .find(|r| r.repo_id == repo_id && r.id == id);

    if let Some(r) = release {
        let asset_id = (r.assets.len() as u64) + 1;
        let asset = ReleaseAsset {
            id: asset_id,
            name: format!("asset-{}.zip", asset_id),
            size: 1024,
            download_url: format!(
                "/api/v1/repos/{}/{}/releases/{}/assets/{}",
                owner, repo_name, id, asset_id
            ),
            created_at: "now".to_string(),
        };
        r.assets.push(asset.clone());
        return (StatusCode::CREATED, Json(Some(asset)));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn download_release_asset(
    State(state): State<AppState>,
    Path((owner, repo_name, id, asset_id)): Path<(String, String, u64, u64)>,
) -> (StatusCode, String) {
    let repos = state.repos.read().unwrap();
    let repo_id = repos
        .iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .map(|r| r.id)
        .unwrap_or(0);

    let releases = state.releases.read().unwrap();
    if let Some(release) = releases.iter().find(|r| r.repo_id == repo_id && r.id == id) {
        if let Some(asset) = release.assets.iter().find(|a| a.id == asset_id) {
            // Mock file content download
            return (
                StatusCode::OK,
                format!("Content of asset {} ({})", asset.name, asset_id),
            );
        }
    }
    (StatusCode::NOT_FOUND, "Asset not found".to_string())
}

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use shared::{Package, CreatePackageOption, Activity};
use crate::router::AppState;

pub async fn list_packages(State(state): State<AppState>, Path(owner): Path<String>) -> Json<Vec<Package>> {
    let packages = state.packages.read().unwrap();
    let filtered_packages: Vec<Package> = packages.iter().filter(|p| p.owner == owner).cloned().collect();
    Json(filtered_packages)
}

pub async fn upload_package(
    State(state): State<AppState>,
    Path(owner): Path<String>,
    Json(payload): Json<CreatePackageOption>
) -> (StatusCode, Json<Package>) {
    let mut packages = state.packages.write().unwrap();
    if packages.iter().any(|p| p.owner == owner && p.name == payload.name && p.version == payload.version) {
        return (StatusCode::CONFLICT, Json(Package { id: 0, owner: "".to_string(), name: "".to_string(), version: "".to_string(), package_type: "".to_string() }));
    }

    let id = (packages.len() as u64) + 1;
    let package = Package {
        id,
        owner: owner.clone(),
        name: payload.name,
        version: payload.version,
        package_type: payload.package_type,
    };
    packages.push(package.clone());

    let mut activities = state.activities.write().unwrap();
    let activity_id = (activities.len() as u64) + 1;
    activities.push(Activity {
        id: activity_id,
        repo_id: 0,
        user_id: 1, // mock admin
        user_name: "admin".to_string(),
        op_type: "upload_package".to_string(),
        content: format!("uploaded package {} ({}) to {}", package.name, package.version, owner),
        created: "now".to_string(),
    });

    (StatusCode::CREATED, Json(package))
}

pub async fn get_package_detail(
    State(state): State<AppState>,
    Path((owner, _type, name, version)): Path<(String, String, String, String)>
) -> Json<Option<Package>> {
    let packages = state.packages.read().unwrap();
    let pkg = packages.iter().find(|p| p.owner == owner && p.name == name && p.version == version).cloned();
    Json(pkg)
}

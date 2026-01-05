use super::AppState;
use codeza_git_service::{CreateRepositoryRequest, Repository};

#[utoipa::path(
    post,
    path = "/api/v1/repos",
    request_body = CreateRepositoryRequest,
    responses(
        (status = 201, description = "Repository created", body = Repository),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "git"
)]
pub async fn create_repository(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::Json(req): axum::Json<codeza_git_service::CreateRepositoryRequest>,
) -> Result<
    (
        axum::http::StatusCode,
        axum::Json<codeza_git_service::Repository>,
    ),
    codeza_shared::CodezaError,
> {
    let repo = state
        .git_service
        .create_repository(req)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok((axum::http::StatusCode::CREATED, axum::Json(repo)))
}

#[utoipa::path(
    get,
    path = "/api/v1/repos/{owner}/{repo}",
    params(
        ("owner" = String, Path, description = "Repository owner"),
        ("repo" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 200, description = "Repository details", body = Repository),
        (status = 404, description = "Repository not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "git"
)]
pub async fn get_repository(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path((owner, repo)): axum::extract::Path<(String, String)>,
) -> Result<axum::Json<codeza_git_service::Repository>, codeza_shared::CodezaError> {
    let repo = state
        .git_service
        .get_repository(&owner, &repo)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok(axum::Json(repo))
}

#[utoipa::path(
    get,
    path = "/api/v1/repos/{owner}",
    params(
        ("owner" = String, Path, description = "Repository owner")
    ),
    responses(
        (status = 200, description = "List of repositories", body = Vec<Repository>),
        (status = 500, description = "Internal server error")
    ),
    tag = "git"
)]
pub async fn list_repositories(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(owner): axum::extract::Path<String>,
) -> Result<axum::Json<Vec<codeza_git_service::Repository>>, codeza_shared::CodezaError> {
    let repos = state
        .git_service
        .list_repositories(&owner)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok(axum::Json(repos))
}

#[utoipa::path(
    delete,
    path = "/api/v1/repos/{owner}/{repo}",
    params(
        ("owner" = String, Path, description = "Repository owner"),
        ("repo" = String, Path, description = "Repository name")
    ),
    responses(
        (status = 204, description = "Repository deleted"),
        (status = 404, description = "Repository not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "git"
)]
pub async fn delete_repository(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path((owner, repo)): axum::extract::Path<(String, String)>,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    state
        .git_service
        .delete_repository(&owner, &repo)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub fn build_git_provider_config(
    config: &codeza_shared::Config,
) -> Result<codeza_git_service::ProviderConfig, codeza_shared::CodezaError> {
    let provider_str = config.git.provider.to_lowercase();

    let provider_type = match provider_str.as_str() {
        "gitea" => codeza_git_service::ProviderType::Gitea,
        "gitlab" => codeza_git_service::ProviderType::GitLab,
        "github" => codeza_git_service::ProviderType::GitHub,
        other => {
            return Err(codeza_shared::CodezaError::ConfigError(format!(
                "Unsupported GIT_PROVIDER value: {}",
                other
            )));
        }
    };

    Ok(codeza_git_service::ProviderConfig::new(
        provider_type,
        config.git.base_url.clone(),
        config.git.access_token.clone(),
    ))
}

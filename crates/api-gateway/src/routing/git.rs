use super::AppState;

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
    let provider_config = build_git_provider_config(&state.config)?;
    let provider = codeza_git_service::create_git_provider(provider_config);
    let service = codeza_git_service::RepositoryService::new(provider);

    let repo = service
        .create_repository(req)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok((axum::http::StatusCode::CREATED, axum::Json(repo)))
}

pub async fn get_repository(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path((owner, repo)):
        axum::extract::Path<(String, String)>,
) -> Result<axum::Json<codeza_git_service::Repository>, codeza_shared::CodezaError> {
    let provider_config = build_git_provider_config(&state.config)?;
    let provider = codeza_git_service::create_git_provider(provider_config);
    let service = codeza_git_service::RepositoryService::new(provider);

    let repo = service
        .get_repository(&owner, &repo)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok(axum::Json(repo))
}

pub async fn list_repositories(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(owner): axum::extract::Path<String>,
) -> Result<axum::Json<Vec<codeza_git_service::Repository>>, codeza_shared::CodezaError> {
    let provider_config = build_git_provider_config(&state.config)?;
    let provider = codeza_git_service::create_git_provider(provider_config);
    let service = codeza_git_service::RepositoryService::new(provider);

    let repos = service
        .list_repositories(&owner)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok(axum::Json(repos))
}

pub async fn delete_repository(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path((owner, repo)):
        axum::extract::Path<(String, String)>,
) -> Result<axum::http::StatusCode, codeza_shared::CodezaError> {
    let provider_config = build_git_provider_config(&state.config)?;
    let provider = codeza_git_service::create_git_provider(provider_config);
    let service = codeza_git_service::RepositoryService::new(provider);

    service
        .delete_repository(&owner, &repo)
        .await
        .map_err(codeza_shared::CodezaError::GitError)?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub(crate) fn build_git_provider_config(
    config: &codeza_shared::Config,
) -> Result<codeza_git_service::ProviderConfig, codeza_shared::CodezaError> {
    let provider_str = config.git.provider.to_lowercase();

    let provider_type = match provider_str.as_str() {
        "gitea" => codeza_git_service::ProviderType::Gitea,
        "gitlab" => codeza_git_service::ProviderType::GitLab,
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

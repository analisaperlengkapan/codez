use utoipa::OpenApi;
use codeza_shared::{LoginRequest, LoginResponse, RegisterRequest, UserResponse};
use codeza_mfe_manager::MicroFrontend;
use codeza_git_service::{CreateRepositoryRequest, Repository};
use codeza_cicd_engine::{PipelineExecutionRecord, JobExecutionRecord};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routing::auth::auth_register,
        crate::routing::auth::auth_login,
        crate::routing::auth::auth_user,
        crate::routing::git::create_repository,
        crate::routing::git::list_repositories,
        crate::routing::git::get_repository,
        crate::routing::git::delete_repository,
        crate::routing::cicd::list_pipelines,
        crate::routing::cicd::get_pipeline_execution,
        crate::routing::cicd::list_pipeline_jobs,
        crate::routing::mfe::list_mfes,
        crate::routing::mfe::register_mfe,
    ),
    components(
        schemas(
            LoginRequest, LoginResponse, RegisterRequest, UserResponse,
            MicroFrontend,
            CreateRepositoryRequest, Repository,
            PipelineExecutionRecord, JobExecutionRecord
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "git", description = "Git repository management"),
        (name = "cicd", description = "CI/CD pipeline operations"),
        (name = "mfe", description = "Micro Frontend management")
    )
)]
pub struct ApiDoc;

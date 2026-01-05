use codeza_cicd_engine::{JobExecutionRecord, PipelineExecutionRecord};
use codeza_git_service::{CreateRepositoryRequest, Repository};
use codeza_mfe_manager::MicroFrontend;
use codeza_mfe_manager::mfe::{MFEManifest, SharedConfig, SharedDependency};
use codeza_msr::Microservice;
use codeza_orchestrator::{AppModule, SuperApp};
use codeza_registry::image::Image;
use codeza_shared::{LoginRequest, LoginResponse, RegisterRequest, UserResponse};
use utoipa::OpenApi;

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
        crate::routing::registry::list_images,
        crate::routing::registry::get_image,
        crate::routing::msr::list_services,
        crate::routing::msr::register_service,
        crate::routing::orchestrator::list_superapps,
        crate::routing::orchestrator::create_superapp,
        crate::routing::orchestrator::get_superapp,
        crate::routing::orchestrator::add_module,
        crate::routing::orchestrator::get_manifest,
    ),
    components(
        schemas(
            LoginRequest, LoginResponse, RegisterRequest, UserResponse,
            MicroFrontend, SharedDependency, MFEManifest, SharedConfig,
            CreateRepositoryRequest, Repository,
            PipelineExecutionRecord, JobExecutionRecord,
            Image, Microservice, SuperApp, AppModule
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "git", description = "Git repository management"),
        (name = "cicd", description = "CI/CD pipeline operations"),
        (name = "mfe", description = "Micro Frontend management"),
        (name = "registry", description = "Container Registry operations"),
        (name = "msr", description = "MicroService Registry operations"),
        (name = "orchestrator", description = "SuperApp Orchestrator operations")
    )
)]
pub struct ApiDoc;

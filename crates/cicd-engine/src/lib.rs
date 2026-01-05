//! Codeza CI/CD Engine
//! Handles pipeline orchestration, job execution, and artifact management

pub mod artifact;
pub mod config_loader;
pub mod db;
pub mod executor;
pub mod pipeline;
pub mod trigger;

pub use artifact::{Artifact, ArtifactStorage, LocalArtifactStorage, S3ArtifactStorage};
pub use config_loader::load_yaml_pipeline_config;
pub use db::{JobExecutionRecord, PipelineExecutionRecord, PipelineExecutionRepository};
pub use executor::{DockerJobExecutor, JobExecutor, LocalJobExecutor};
pub use pipeline::{Job, Pipeline, PipelineExecution, PipelineStatus, Stage};
pub use trigger::{
    GitPushContext, TriggerResult, pipeline_from_yaml_str, trigger_push_pipeline,
    trigger_push_pipeline_from_yaml,
};

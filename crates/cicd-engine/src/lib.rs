//! Codeza CI/CD Engine
//! Handles pipeline orchestration, job execution, and artifact management

pub mod pipeline;
pub mod executor;
pub mod artifact;
pub mod trigger;
pub mod db;
pub mod config_loader;

pub use pipeline::{Pipeline, Stage, Job, PipelineExecution, PipelineStatus};
pub use executor::{JobExecutor, LocalJobExecutor, DockerJobExecutor};
pub use artifact::{Artifact, ArtifactStorage, LocalArtifactStorage, S3ArtifactStorage};
pub use trigger::{GitPushContext, TriggerResult, trigger_push_pipeline, pipeline_from_yaml_str, trigger_push_pipeline_from_yaml};
pub use db::{PipelineExecutionRepository, PipelineExecutionRecord, JobExecutionRecord};
pub use config_loader::load_yaml_pipeline_config;

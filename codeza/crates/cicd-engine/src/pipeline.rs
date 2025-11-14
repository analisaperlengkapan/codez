//! Pipeline definition and models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Pipeline definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub stages: Vec<Stage>,
    pub variables: Option<HashMap<String, String>>,
    pub triggers: Option<Vec<Trigger>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Pipeline stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    pub description: Option<String>,
    pub jobs: Vec<Job>,
    pub when: Option<String>, // always, on_success, on_failure
}

/// Job definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub script: Vec<String>,
    pub before_script: Option<Vec<String>>,
    pub after_script: Option<Vec<String>>,
    pub artifacts: Option<Artifacts>,
    pub cache: Option<Cache>,
    pub variables: Option<HashMap<String, String>>,
    pub timeout: Option<u64>, // seconds
    pub retry: Option<u32>,
    pub allow_failure: Option<bool>,
}

/// Artifacts configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifacts {
    pub paths: Vec<String>,
    pub exclude: Option<Vec<String>>,
    pub expire_in: Option<String>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub paths: Vec<String>,
    pub key: Option<String>,
}

/// Pipeline trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub event: String, // push, pull_request, schedule, manual
    pub branch: Option<String>,
    pub tag: Option<String>,
}

/// Pipeline status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
    Skipped,
}

/// Pipeline execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineExecution {
    pub id: Uuid,
    pub pipeline_id: Uuid,
    pub status: PipelineStatus,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub stages: Vec<StageExecution>,
}

/// Stage execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageExecution {
    pub name: String,
    pub status: PipelineStatus,
    pub jobs: Vec<JobExecution>,
}

/// Job execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecution {
    pub id: Uuid,
    pub name: String,
    pub status: PipelineStatus,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<u64>, // seconds
    pub log_url: Option<String>,
}

impl Pipeline {
    /// Create new pipeline
    pub fn new(name: String, stages: Vec<Stage>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            stages,
            variables: None,
            triggers: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl PipelineExecution {
    /// Create new pipeline execution
    pub fn new(pipeline_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            pipeline_id,
            status: PipelineStatus::Pending,
            started_at: None,
            finished_at: None,
            stages: Vec::new(),
        }
    }

    /// Start execution
    pub fn start(&mut self) {
        self.status = PipelineStatus::Running;
        self.started_at = Some(chrono::Utc::now());
    }

    /// Mark as success
    pub fn success(&mut self) {
        self.status = PipelineStatus::Success;
        self.finished_at = Some(chrono::Utc::now());
    }

    /// Mark as failed
    pub fn failed(&mut self) {
        self.status = PipelineStatus::Failed;
        self.finished_at = Some(chrono::Utc::now());
    }

    /// Get duration in seconds
    pub fn duration(&self) -> Option<u64> {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => {
                let duration = end.signed_duration_since(start);
                Some(duration.num_seconds() as u64)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = Pipeline::new("test-pipeline".to_string(), vec![]);
        assert_eq!(pipeline.name, "test-pipeline");
        assert!(pipeline.stages.is_empty());
    }

    #[test]
    fn test_pipeline_execution() {
        let pipeline_id = Uuid::new_v4();
        let mut execution = PipelineExecution::new(pipeline_id);
        
        assert_eq!(execution.status, PipelineStatus::Pending);
        
        execution.start();
        assert_eq!(execution.status, PipelineStatus::Running);
        
        execution.success();
        assert_eq!(execution.status, PipelineStatus::Success);
    }
}

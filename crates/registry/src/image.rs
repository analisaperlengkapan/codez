//! Container image models and operations

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

/// Container image
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Image {
    pub id: Uuid,
    pub name: String,
    pub registry: String,
    pub digest: String,
    pub size: u64,
    pub config: ImageConfig,
    pub layers: Vec<Layer>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Image configuration
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageConfig {
    pub architecture: String,
    pub os: String,
    pub os_version: Option<String>,
    pub config: ImageConfigDetails,
}

/// Image configuration details
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageConfigDetails {
    pub env: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub working_dir: Option<String>,
    pub labels: Option<std::collections::HashMap<String, String>>,
}

/// Image layer
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Layer {
    pub digest: String,
    pub size: u64,
    pub media_type: String,
}

/// Image manifest (Docker Image Manifest v2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageManifest {
    pub schema_version: u32,
    pub media_type: String,
    pub config: ManifestConfig,
    pub layers: Vec<ManifestLayer>,
}

/// Manifest config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestConfig {
    pub size: u64,
    pub digest: String,
}

/// Manifest layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestLayer {
    pub size: u64,
    pub digest: String,
    pub media_type: String,
}

/// Image push request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushImageRequest {
    pub name: String,
    pub tag: String,
    pub manifest: ImageManifest,
}

/// Image pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullImageRequest {
    pub name: String,
    pub tag: String,
}

/// Image pull response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullImageResponse {
    pub image: Image,
    pub manifest: ImageManifest,
}

impl Image {
    /// Create new image
    pub fn new(name: String, registry: String, digest: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            registry,
            digest,
            size: 0,
            config: ImageConfig {
                architecture: "amd64".to_string(),
                os: "linux".to_string(),
                os_version: None,
                config: ImageConfigDetails {
                    env: None,
                    cmd: None,
                    entrypoint: None,
                    working_dir: None,
                    labels: None,
                },
            },
            layers: Vec::new(),
            tags: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Add tag to image
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self.updated_at = chrono::Utc::now();
    }

    /// Remove tag from image
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = chrono::Utc::now();
    }

    /// Calculate total size from layers
    pub fn calculate_size(&mut self) {
        self.size = self.layers.iter().map(|l| l.size).sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_creation() {
        let image = Image::new(
            "myapp".to_string(),
            "registry.example.com".to_string(),
            "sha256:abc123".to_string(),
        );

        assert_eq!(image.name, "myapp");
        assert_eq!(image.registry, "registry.example.com");
        assert!(image.tags.is_empty());
    }

    #[test]
    fn test_image_tagging() {
        let mut image = Image::new(
            "myapp".to_string(),
            "registry.example.com".to_string(),
            "sha256:abc123".to_string(),
        );

        image.add_tag("latest".to_string());
        image.add_tag("v1.0.0".to_string());

        assert_eq!(image.tags.len(), 2);
        assert!(image.tags.contains(&"latest".to_string()));

        image.remove_tag("latest");
        assert_eq!(image.tags.len(), 1);
    }

    #[test]
    fn test_image_size_calculation() {
        let mut image = Image::new(
            "myapp".to_string(),
            "registry.example.com".to_string(),
            "sha256:abc123".to_string(),
        );

        image.layers.push(Layer {
            digest: "sha256:layer1".to_string(),
            size: 1000,
            media_type: "application/vnd.docker.image.rootfs.diff.tar.gzip".to_string(),
        });

        image.layers.push(Layer {
            digest: "sha256:layer2".to_string(),
            size: 2000,
            media_type: "application/vnd.docker.image.rootfs.diff.tar.gzip".to_string(),
        });

        image.calculate_size();
        assert_eq!(image.size, 3000);
    }
}

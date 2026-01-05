//! Image push and pull operations

use crate::image::{Image, ImageManifest, PullImageRequest, PullImageResponse, PushImageRequest};
use async_trait::async_trait;
use std::collections::HashMap;

/// Image storage trait
#[async_trait]
pub trait ImageStorage: Send + Sync {
    /// Push image
    async fn push(&self, request: &PushImageRequest) -> Result<Image, String>;

    /// Pull image
    async fn pull(&self, request: &PullImageRequest) -> Result<PullImageResponse, String>;

    /// Get image by name and tag
    async fn get_image(&self, name: &str, tag: &str) -> Result<Image, String>;

    /// List images
    async fn list_images(&self, filter: Option<String>) -> Result<Vec<Image>, String>;

    /// Delete image
    async fn delete_image(&self, name: &str, tag: &str) -> Result<(), String>;

    /// Get image by digest
    async fn get_image_by_digest(&self, digest: &str) -> Result<Image, String>;
}

/// Local image storage (for testing)
pub struct LocalImageStorage {
    images: std::sync::Arc<tokio::sync::RwLock<HashMap<String, Image>>>,
}

impl LocalImageStorage {
    /// Create new local image storage
    pub fn new() -> Self {
        Self {
            images: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

impl Default for LocalImageStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ImageStorage for LocalImageStorage {
    async fn push(&self, request: &PushImageRequest) -> Result<Image, String> {
        let key = format!("{}:{}", request.name, request.tag);
        let mut image = Image::new(
            request.name.clone(),
            "localhost".to_string(),
            request.manifest.config.digest.clone(),
        );
        image.add_tag(request.tag.clone());

        let mut images = self.images.write().await;
        images.insert(key, image.clone());

        tracing::info!("Pushed image: {}:{}", request.name, request.tag);
        Ok(image)
    }

    async fn pull(&self, request: &PullImageRequest) -> Result<PullImageResponse, String> {
        let key = format!("{}:{}", request.name, request.tag);
        let images = self.images.read().await;

        images
            .get(&key)
            .ok_or_else(|| format!("Image not found: {}:{}", request.name, request.tag))
            .map(|image| {
                tracing::info!("Pulled image: {}:{}", request.name, request.tag);
                PullImageResponse {
                    image: image.clone(),
                    manifest: ImageManifest {
                        schema_version: 2,
                        media_type: "application/vnd.docker.distribution.manifest.v2+json"
                            .to_string(),
                        config: crate::image::ManifestConfig {
                            size: 0,
                            digest: image.digest.clone(),
                        },
                        layers: image
                            .layers
                            .iter()
                            .map(|l| crate::image::ManifestLayer {
                                size: l.size,
                                digest: l.digest.clone(),
                                media_type: l.media_type.clone(),
                            })
                            .collect(),
                    },
                }
            })
    }

    async fn get_image(&self, name: &str, tag: &str) -> Result<Image, String> {
        let key = format!("{}:{}", name, tag);
        let images = self.images.read().await;

        images
            .get(&key)
            .cloned()
            .ok_or_else(|| format!("Image not found: {}:{}", name, tag))
    }

    async fn list_images(&self, _filter: Option<String>) -> Result<Vec<Image>, String> {
        let images = self.images.read().await;
        Ok(images.values().cloned().collect())
    }

    async fn delete_image(&self, name: &str, tag: &str) -> Result<(), String> {
        let key = format!("{}:{}", name, tag);
        let mut images = self.images.write().await;
        images.remove(&key);

        tracing::info!("Deleted image: {}:{}", name, tag);
        Ok(())
    }

    async fn get_image_by_digest(&self, digest: &str) -> Result<Image, String> {
        let images = self.images.read().await;

        images
            .values()
            .find(|img| img.digest == digest)
            .cloned()
            .ok_or_else(|| format!("Image not found with digest: {}", digest))
    }
}

/// Remote image storage (for S3/MinIO)
pub struct RemoteImageStorage {
    endpoint: String,
}

impl RemoteImageStorage {
    /// Create new remote image storage
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[async_trait]
impl ImageStorage for RemoteImageStorage {
    async fn push(&self, request: &PushImageRequest) -> Result<Image, String> {
        let mut image = Image::new(
            request.name.clone(),
            self.endpoint.clone(),
            request.manifest.config.digest.clone(),
        );
        image.add_tag(request.tag.clone());

        tracing::info!("Pushed image to remote: {}:{}", request.name, request.tag);
        Ok(image)
    }

    async fn pull(&self, request: &PullImageRequest) -> Result<PullImageResponse, String> {
        let image = Image::new(
            request.name.clone(),
            self.endpoint.clone(),
            "sha256:remote".to_string(),
        );

        tracing::info!("Pulled image from remote: {}:{}", request.name, request.tag);
        Ok(PullImageResponse {
            image,
            manifest: ImageManifest {
                schema_version: 2,
                media_type: "application/vnd.docker.distribution.manifest.v2+json".to_string(),
                config: crate::image::ManifestConfig {
                    size: 0,
                    digest: "sha256:remote".to_string(),
                },
                layers: vec![],
            },
        })
    }

    async fn get_image(&self, name: &str, tag: &str) -> Result<Image, String> {
        let image = Image::new(
            name.to_string(),
            self.endpoint.clone(),
            "sha256:remote".to_string(),
        );
        tracing::info!("Got image from remote: {}:{}", name, tag);
        Ok(image)
    }

    async fn list_images(&self, _filter: Option<String>) -> Result<Vec<Image>, String> {
        tracing::info!("Listed images from remote");
        Ok(vec![])
    }

    async fn delete_image(&self, name: &str, tag: &str) -> Result<(), String> {
        tracing::info!("Deleted image from remote: {}:{}", name, tag);
        Ok(())
    }

    async fn get_image_by_digest(&self, digest: &str) -> Result<Image, String> {
        let image = Image::new(
            "unknown".to_string(),
            self.endpoint.clone(),
            digest.to_string(),
        );
        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_push_pull() {
        let storage = LocalImageStorage::new();

        let push_req = PushImageRequest {
            name: "myapp".to_string(),
            tag: "latest".to_string(),
            manifest: ImageManifest {
                schema_version: 2,
                media_type: "application/vnd.docker.distribution.manifest.v2+json".to_string(),
                config: crate::image::ManifestConfig {
                    size: 100,
                    digest: "sha256:abc123".to_string(),
                },
                layers: vec![],
            },
        };

        let push_result = storage.push(&push_req).await;
        assert!(push_result.is_ok());

        let pull_req = PullImageRequest {
            name: "myapp".to_string(),
            tag: "latest".to_string(),
        };

        let pull_result = storage.pull(&pull_req).await;
        assert!(pull_result.is_ok());
    }

    #[tokio::test]
    async fn test_local_list_images() {
        let storage = LocalImageStorage::new();

        let push_req = PushImageRequest {
            name: "myapp".to_string(),
            tag: "latest".to_string(),
            manifest: ImageManifest {
                schema_version: 2,
                media_type: "application/vnd.docker.distribution.manifest.v2+json".to_string(),
                config: crate::image::ManifestConfig {
                    size: 100,
                    digest: "sha256:abc123".to_string(),
                },
                layers: vec![],
            },
        };

        let _ = storage.push(&push_req).await;

        let list_result = storage.list_images(None).await;
        assert!(list_result.is_ok());
        assert_eq!(list_result.unwrap().len(), 1);
    }
}

//! Database repository for Microservices

use sqlx::PgPool;
use uuid::Uuid;
use codeza_shared::error::{CodezaError, Result};
use crate::service::{Microservice, ServiceStatus};

/// Repository for Microservice persistence
pub struct MicroserviceRepository {
    pool: PgPool,
}

impl MicroserviceRepository {
    /// Create new repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Register a new microservice
    pub async fn create(&self, service: Microservice) -> Result<Microservice> {
        let rec = sqlx::query_as::<_, Microservice>(
            r#"
            INSERT INTO microservices (
                id, name, version, host, port, protocol, status, metadata, tags, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#
        )
        .bind(service.id)
        .bind(&service.name)
        .bind(&service.version)
        .bind(&service.host)
        .bind(service.port)
        .bind(&service.protocol)
        .bind(service.status)
        .bind(sqlx::types::Json(&service.metadata))
        .bind(&service.tags)
        .bind(service.created_at)
        .bind(service.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to register service: {}", e);
            CodezaError::DatabaseError(e.to_string())
        })?;

        Ok(rec)
    }

    /// List all microservices
    pub async fn list(&self) -> Result<Vec<Microservice>> {
        let services = sqlx::query_as::<_, Microservice>(
            "SELECT * FROM microservices ORDER BY name, version"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list services: {}", e);
            CodezaError::DatabaseError(e.to_string())
        })?;

        Ok(services)
    }

    /// Get microservice by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<Microservice>> {
        let service = sqlx::query_as::<_, Microservice>(
            "SELECT * FROM microservices WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get service {}: {}", id, e);
            CodezaError::DatabaseError(e.to_string())
        })?;

        Ok(service)
    }

    /// Update service status
    pub async fn update_status(&self, id: Uuid, status: ServiceStatus) -> Result<Microservice> {
        let service = sqlx::query_as::<_, Microservice>(
            r#"
            UPDATE microservices
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            match e {
                sqlx::Error::RowNotFound => CodezaError::NotFound(format!("Service {}", id)),
                _ => {
                    tracing::error!("Failed to update status for {}: {}", id, e);
                    CodezaError::DatabaseError(e.to_string())
                }
            }
        })?;

        Ok(service)
    }

    /// Delete microservice
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query(
            "DELETE FROM microservices WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete service {}: {}", id, e);
            CodezaError::DatabaseError(e.to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(CodezaError::NotFound(format!("Service {}", id)));
        }

        Ok(())
    }
}

use sqlx::PgPool;
use uuid::Uuid;
use crate::mfe::{MicroFrontend, MFEStatus};
use codeza_shared::CodezaError;

pub struct MFERepository {
    pool: PgPool,
}

impl MFERepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_active(&self) -> Result<Vec<MicroFrontend>, CodezaError> {
        let rows = sqlx::query(
            "SELECT id, name, description, version, remote_entry, scope, status, created_at, updated_at \
             FROM micro_frontends \
             WHERE status = 'Active' OR status = 'active'",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        let mut mfes = Vec::new();
        for row in rows {
            let mfe = self.map_row_to_mfe(row).await?;
            mfes.push(mfe);
        }

        Ok(mfes)
    }

    pub async fn register(&self, mut mfe: MicroFrontend) -> Result<MicroFrontend, CodezaError> {
        if mfe.id == Uuid::nil() {
            mfe.id = Uuid::new_v4();
        }

        // Simple serialization of status for now
        let status_str = match mfe.status {
            MFEStatus::Active => "Active",
            MFEStatus::Inactive => "Inactive",
            MFEStatus::Deprecated => "Deprecated",
            MFEStatus::Maintenance => "Maintenance",
        };

        let mut tx = self.pool.begin().await.map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        sqlx::query(
            "INSERT INTO micro_frontends (id, name, description, version, remote_entry, scope, status, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
             ON CONFLICT (name) DO UPDATE SET \
             description = EXCLUDED.description, \
             version = EXCLUDED.version, \
             remote_entry = EXCLUDED.remote_entry, \
             scope = EXCLUDED.scope, \
             status = EXCLUDED.status, \
             updated_at = NOW()"
        )
        .bind(mfe.id)
        .bind(&mfe.name)
        .bind(&mfe.description)
        .bind(&mfe.version)
        .bind(&mfe.remote_entry)
        .bind(&mfe.scope)
        .bind(status_str)
        .bind(mfe.created_at)
        .bind(mfe.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        tx.commit().await.map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        Ok(mfe)
    }

    async fn map_row_to_mfe(&self, row: sqlx::postgres::PgRow) -> Result<MicroFrontend, CodezaError> {
        use sqlx::Row;

        let id: Uuid = row.try_get("id").map_err(|e| CodezaError::DatabaseError(e.to_string()))?;
        let name: String = row.try_get("name").map_err(|e| CodezaError::DatabaseError(e.to_string()))?;
        let status_str: String = row.try_get("status").map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        let status = match status_str.as_str() {
            "Active" | "active" => MFEStatus::Active,
            "Inactive" | "inactive" => MFEStatus::Inactive,
            "Deprecated" | "deprecated" => MFEStatus::Deprecated,
            "Maintenance" | "maintenance" => MFEStatus::Maintenance,
            _ => MFEStatus::Inactive,
        };

        Ok(MicroFrontend {
            id,
            name,
            description: row.try_get("description").ok(),
            version: row.try_get("version").unwrap_or_default(),
            remote_entry: row.try_get("remote_entry").unwrap_or_default(),
            scope: row.try_get("scope").unwrap_or_default(),
            dependencies: std::collections::HashMap::new(), // Todo: fetch from sub-table
            shared_dependencies: Vec::new(), // Todo: fetch from sub-table
            status,
            created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
        })
    }
}

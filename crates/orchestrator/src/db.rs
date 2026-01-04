use sqlx::PgPool;
use uuid::Uuid;
use crate::superapp::{SuperApp, AppModule, AppConfig};
use codeza_shared::CodezaError;

pub struct SuperAppRepository {
    pool: PgPool,
}

impl SuperAppRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<SuperApp>, CodezaError> {
        // Fetch apps and modules in a single query using JSON aggregation to avoid N+1
        let rows = sqlx::query(
            r#"
            SELECT
                sa.id, sa.name, sa.version, sa.description, sa.config, sa.created_at, sa.updated_at,
                COALESCE(
                    JSONB_AGG(
                        JSONB_BUILD_OBJECT(
                            'id', sam.id,
                            'name', sam.name,
                            'version', sam.version,
                            'remote_entry', sam.remote_entry,
                            'scope', sam.scope,
                            'dependencies', sam.dependencies
                        )
                    ) FILTER (WHERE sam.id IS NOT NULL),
                    '[]'::jsonb
                ) as modules
            FROM super_apps sa
            LEFT JOIN super_app_modules sam ON sa.id = sam.super_app_id
            GROUP BY sa.id
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        let mut apps = Vec::new();
        for row in rows {
            apps.push(self.map_row_to_superapp_with_modules(row)?);
        }

        Ok(apps)
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<SuperApp>, CodezaError> {
        let row = sqlx::query(
            r#"
            SELECT
                sa.id, sa.name, sa.version, sa.description, sa.config, sa.created_at, sa.updated_at,
                COALESCE(
                    JSONB_AGG(
                        JSONB_BUILD_OBJECT(
                            'id', sam.id,
                            'name', sam.name,
                            'version', sam.version,
                            'remote_entry', sam.remote_entry,
                            'scope', sam.scope,
                            'dependencies', sam.dependencies
                        )
                    ) FILTER (WHERE sam.id IS NOT NULL),
                    '[]'::jsonb
                ) as modules
            FROM super_apps sa
            LEFT JOIN super_app_modules sam ON sa.id = sam.super_app_id
            WHERE sa.id = $1
            GROUP BY sa.id
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(self.map_row_to_superapp_with_modules(row)?)),
            None => Ok(None),
        }
    }

    pub async fn create(&self, app: SuperApp) -> Result<SuperApp, CodezaError> {
        let mut tx = self.pool.begin().await.map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        sqlx::query(
            "INSERT INTO super_apps (id, name, version, description, config, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(app.id)
        .bind(&app.name)
        .bind(&app.version)
        .bind(&app.description)
        .bind(sqlx::types::Json(&app.config))
        .bind(app.created_at)
        .bind(app.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        // Handle modules if provided in the input
        for module in &app.modules {
             sqlx::query(
                "INSERT INTO super_app_modules (id, super_app_id, name, version, remote_entry, scope, dependencies, created_at) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())"
            )
            .bind(module.id)
            .bind(app.id)
            .bind(&module.name)
            .bind(&module.version)
            .bind(&module.remote_entry)
            .bind(&module.scope)
            .bind(sqlx::types::Json(&module.dependencies))
            .execute(&mut *tx)
            .await
            .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        Ok(app)
    }

    pub async fn add_module(&self, super_app_id: Uuid, mut module: AppModule) -> Result<AppModule, CodezaError> {
        if module.id == Uuid::nil() {
            module.id = Uuid::new_v4();
        }

        sqlx::query(
            "INSERT INTO super_app_modules (id, super_app_id, name, version, remote_entry, scope, dependencies, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())"
        )
        .bind(module.id)
        .bind(super_app_id)
        .bind(&module.name)
        .bind(&module.version)
        .bind(&module.remote_entry)
        .bind(&module.scope)
        .bind(sqlx::types::Json(&module.dependencies))
        .execute(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        Ok(module)
    }

    pub async fn remove_module(&self, super_app_id: Uuid, module_id: Uuid) -> Result<(), CodezaError> {
        sqlx::query(
            "DELETE FROM super_app_modules WHERE id = $1 AND super_app_id = $2"
        )
        .bind(module_id)
        .bind(super_app_id)
        .execute(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    fn map_row_to_superapp_with_modules(&self, row: sqlx::postgres::PgRow) -> Result<SuperApp, CodezaError> {
        use sqlx::Row;

        let id: Uuid = row.try_get("id").map_err(|e| CodezaError::DatabaseError(e.to_string()))?;
        let config_json: sqlx::types::Json<AppConfig> = row.try_get("config").map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        let modules_json: sqlx::types::Json<Vec<serde_json::Value>> = row.try_get("modules").map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        let mut modules = Vec::new();
        for m_val in modules_json.0 {
            let dependencies: std::collections::HashMap<String, String> = serde_json::from_value(m_val["dependencies"].clone())
                .map_err(|e| CodezaError::InternalError(e.to_string()))?;

            modules.push(AppModule {
                id: serde_json::from_value(m_val["id"].clone()).map_err(|e| CodezaError::InternalError(e.to_string()))?,
                name: serde_json::from_value(m_val["name"].clone()).map_err(|e| CodezaError::InternalError(e.to_string()))?,
                version: serde_json::from_value(m_val["version"].clone()).map_err(|e| CodezaError::InternalError(e.to_string()))?,
                remote_entry: serde_json::from_value(m_val["remote_entry"].clone()).map_err(|e| CodezaError::InternalError(e.to_string()))?,
                scope: serde_json::from_value(m_val["scope"].clone()).map_err(|e| CodezaError::InternalError(e.to_string()))?,
                dependencies,
            });
        }

        Ok(SuperApp {
            id,
            name: row.try_get("name").map_err(|e| CodezaError::DatabaseError(e.to_string()))?,
            version: row.try_get("version").map_err(|e| CodezaError::DatabaseError(e.to_string()))?,
            description: row.try_get("description").ok(),
            modules,
            config: config_json.0,
            created_at: row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
        })
    }
}

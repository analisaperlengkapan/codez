//! User service for database operations

use codeza_shared::{
    error::{CodezaError, Result},
    models::{User, UserResponse, RegisterRequest},
    auth::{hash_password, verify_password},
};
use sqlx::PgPool;
use uuid::Uuid;

/// User service for database operations
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    /// Create new user service
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user
    pub async fn create_user(&self, req: RegisterRequest) -> Result<UserResponse> {
        let user_id = Uuid::new_v4();
        let password_hash = hash_password(&req.password)?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash, full_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, email, password_hash, full_name, avatar_url, 
                      is_active, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(&req.username)
        .bind(&req.email)
        .bind(&password_hash)
        .bind(&req.full_name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                CodezaError::ValidationError("Username or email already exists".to_string())
            } else {
                CodezaError::DatabaseError(e.to_string())
            }
        })?;

        Ok(user.into())
    }

    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, password_hash, full_name, avatar_url,
                   is_active, created_at, updated_at
            FROM users
            WHERE username = $1 AND is_active = true
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?
        .ok_or_else(|| CodezaError::NotFound("User not found".to_string()))
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, username, email, password_hash, full_name, avatar_url,
                   is_active, created_at, updated_at
            FROM users
            WHERE id = $1 AND is_active = true
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?
        .ok_or_else(|| CodezaError::NotFound("User not found".to_string()))
    }

    /// Verify user credentials
    pub async fn verify_credentials(&self, username: &str, password: &str) -> Result<User> {
        let user = self.get_user_by_username(username).await?;
        let is_valid = verify_password(password, &user.password_hash)?;
        
        if !is_valid {
            return Err(CodezaError::AuthenticationError(
                "Invalid credentials".to_string(),
            ));
        }

        Ok(user)
    }

    /// Get user roles
    pub async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>> {
        let roles = sqlx::query_scalar::<_, String>(
            r#"
            SELECT r.name
            FROM roles r
            JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| CodezaError::DatabaseError(e.to_string()))?;

        Ok(roles)
    }
}

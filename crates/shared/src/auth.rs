//! Authentication and authorization utilities

use crate::error::{CodezaError, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use uuid::Uuid;

use crate::models::JwtClaims;

/// Hash password using Argon2
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(rand::thread_rng());
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| CodezaError::InternalError("Failed to hash password".to_string()))
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| CodezaError::InternalError("Invalid password hash".to_string()))?;
    
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Generate JWT token
pub fn generate_token(
    user_id: Uuid,
    username: String,
    email: String,
    roles: Vec<String>,
    secret: &str,
    expiration_hours: i64,
) -> Result<String> {
    let now = Utc::now();
    let exp = (now + chrono::Duration::hours(expiration_hours)).timestamp();
    let iat = now.timestamp();
    
    let claims = JwtClaims {
        sub: user_id.to_string(),
        username,
        email,
        roles,
        exp,
        iat,
        nbf: iat,
    };
    
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| CodezaError::InternalError("Failed to generate token".to_string()))
}

/// Verify and decode JWT token
pub fn verify_token(token: &str, secret: &str) -> Result<JwtClaims> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    decode::<JwtClaims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| CodezaError::AuthenticationError("Invalid token".to_string()))
}

/// Generate random refresh token
pub fn generate_refresh_token() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.r#gen()).collect();
    hex::encode(random_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_password() {
        let password = "test_password_123";
        let hash = hash_password(password).unwrap();
        let is_valid = verify_password(password, &hash).unwrap();
        assert!(is_valid);
        
        let is_invalid = verify_password("wrong_password", &hash).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_generate_and_verify_token() {
        let secret = "test_secret_key";
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let email = "test@example.com".to_string();
        let roles = vec!["developer".to_string()];
        
        let token = generate_token(
            user_id,
            username.clone(),
            email.clone(),
            roles.clone(),
            secret,
            24,
        ).unwrap();
        
        let claims = verify_token(&token, secret).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
        assert_eq!(claims.email, email);
        assert_eq!(claims.roles, roles);
    }

    #[test]
    fn test_generate_refresh_token() {
        let token1 = generate_refresh_token();
        let token2 = generate_refresh_token();
        
        assert!(!token1.is_empty());
        assert!(!token2.is_empty());
        assert_ne!(token1, token2);
    }
}

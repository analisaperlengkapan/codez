use crate::auth::verify_token;
use crate::config::Config;
use crate::error::CodezaError;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

/// Middleware to verify JWT token and inject user_id into request extensions
pub async fn auth_middleware(
    State(config): State<Config>,
    mut request: Request,
    next: Next,
) -> Response {
    let token = match request.headers().get(header::AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Ok(s) => {
                if let Some(token) = s.strip_prefix("Bearer ") {
                    token
                } else {
                    return CodezaError::AuthenticationError(
                        "Invalid authorization header format".to_string(),
                    )
                    .into_response();
                }
            }
            Err(_) => {
                return CodezaError::AuthenticationError("Invalid authorization header".to_string())
                    .into_response();
            }
        },
        None => {
            return CodezaError::AuthenticationError("Missing authorization header".to_string())
                .into_response();
        }
    };

    match verify_token(token, &config.jwt.secret) {
        Ok(claims) => {
            if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                request.extensions_mut().insert(user_id);
                // Also insert claims if needed later
                request.extensions_mut().insert(claims);
            } else {
                return CodezaError::AuthenticationError("Invalid user ID in token".to_string())
                    .into_response();
            }
        }
        Err(_) => {
            return CodezaError::AuthenticationError("Invalid token".to_string()).into_response();
        }
    }

    next.run(request).await
}

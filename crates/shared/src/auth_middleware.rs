
use axum::{
    extract::{State, Request},
    middleware::Next,
    response::{Response, IntoResponse},
    http::{StatusCode, header},
};
use crate::config::Config;
use crate::auth::verify_token;
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
                if s.starts_with("Bearer ") {
                    &s[7..]
                } else {
                    return (StatusCode::UNAUTHORIZED, "Invalid authorization header format").into_response();
                }
            }
            Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid authorization header").into_response(),
        },
        None => return (StatusCode::UNAUTHORIZED, "Missing authorization header").into_response(),
    };

    match verify_token(token, &config.jwt.secret) {
        Ok(claims) => {
            if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                request.extensions_mut().insert(user_id);
                // Also insert claims if needed later
                request.extensions_mut().insert(claims);
            } else {
                return (StatusCode::UNAUTHORIZED, "Invalid user ID in token").into_response();
            }
        }
        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
    }

    next.run(request).await
}

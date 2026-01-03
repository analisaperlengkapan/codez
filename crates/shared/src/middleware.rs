//! Common middleware for Codeza Platform

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

/// Middleware to add request ID to all requests
pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    request.extensions_mut().insert(request_id.clone());
    
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    response
}

/// Middleware to log all requests
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    tracing::info!("{} {}", method, uri);
    
    let response = next.run(request).await;
    
    tracing::info!("{} {} - {}", method, uri, response.status());
    
    response
}

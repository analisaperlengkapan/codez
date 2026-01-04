use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;
use uuid::Uuid;

/// Request ID middleware
pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4();
    request.extensions_mut().insert(request_id);

    let response = next.run(request).await;
    
    // Add request ID header to response
    let mut response = response;
    response.headers_mut().insert(
        "x-request-id",
        request_id.to_string().parse().unwrap(),
    );
    
    response
}

/// Logging middleware
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    // Extract request ID if available
    let request_id = request
        .extensions()
        .get::<Uuid>()
        .cloned()
        .unwrap_or_default();

    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        "Request started"
    );
    
    let response = next.run(request).await;
    
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        status = %response.status(),
        "Request completed"
    );
    
    response
}

/// Auth middleware re-export
pub use crate::auth_middleware::auth_middleware;

use std::net::SocketAddr;

mod db;
mod handlers;
mod router;
mod tests;

#[tokio::main]
async fn main() {
    let (api_router, state) = router::api_router_with_state();

    let serve_dir = tower_http::services::ServeDir::new("crates/frontend/dist")
        .not_found_service(tower_http::services::ServeFile::new("crates/frontend/dist/index.html"));

    let app = api_router.fallback_service(serve_dir);

    tokio::spawn(async move {
        db::init_postgres_db(&state).await;
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

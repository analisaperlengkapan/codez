use std::net::SocketAddr;

mod db;
mod handlers;
mod router;
mod tests;

#[tokio::main]
async fn main() {
    let (app, state) = router::api_router_with_state();

    tokio::spawn(async move {
        db::init_postgres_db(&state).await;
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

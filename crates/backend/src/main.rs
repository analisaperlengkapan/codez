use std::net::SocketAddr;

mod handlers;
mod router;
mod tests;

use router::api_router;

#[tokio::main]
async fn main() {
    let app = api_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

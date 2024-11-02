mod routes;
mod models;
mod utils;

use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;
use crate::routes::trees::{create_tree, get_tree_messages, create_tree_message};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let app = Router::new()
        .route("/v1/trees", post(create_tree))
        .route("/v1/trees/:account_hash/messages", get(get_tree_messages))
        .route("/v1/trees/:account_hash/messages", post(create_tree_message))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

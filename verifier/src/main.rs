mod routes;
mod models;

use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;
use crate::routes::trees::{get_tree_messages, create_tree_message};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/trees/:account_hash/messages", get(get_tree_messages))
        .route("/v1/messages", post(create_tree_message));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

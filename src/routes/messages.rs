use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use crate::{db::{connection::DbPool, queries::{get_messages, get_tree}}, utils::proof::ProofJson};
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealTreeMessages {
    pub account_proof: ProofJson,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub hash: String,
    pub ornament_id: i32,
    pub nickname: String,
    pub merkle_root: String,
    pub body: Option<String>,
}

pub async fn get_tree_messages_route(
    State(pool): State<Arc<DbPool>>,
    Path(account_hash): Path<String>
) -> Result<Json<Vec<MessageResponse>>, StatusCode> {

    // Get Existing Messages
    let conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> = &mut pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tree = match get_tree(conn, &account_hash) {
        Ok(tree) => tree,
        Err(diesel::result::Error::NotFound) => {
            return Err(StatusCode::NOT_FOUND);
        },
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    let messages = match get_messages(conn, &account_hash) {   
        Ok(messages) => messages,
        Err(diesel::result::Error::NotFound) => vec![],
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    let response = messages.iter().map(|message| {
        MessageResponse {
            hash: message.hash.clone(),
            ornament_id: message.ornament_id,
            nickname: message.nickname.clone(),
            merkle_root: tree.merkle_root.clone(),
            body: None,
        }
    }).collect();

    Ok(Json(response))
}

#[derive(Deserialize)]
pub struct CreateMessage {
    pub nickname: String,
    pub body: String,
    pub friendship_proof: String,
    pub merkle_root: String,
}


pub async fn create_tree_message_route(Json(payload): Json<CreateMessage>) -> (StatusCode, Json<MessageResponse>) {
    // TODO : Save to DB
    let message = MessageResponse {
        hash: "hash".to_string(),
        ornament_id: 1,
        nickname: payload.nickname,
        merkle_root: payload.merkle_root,
        body: Some(payload.body),
    };

    (StatusCode::CREATED, Json(message))
}
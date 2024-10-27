use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;
use crate::models::{Message, Tree};

#[derive(Deserialize)]
pub struct CreateTree {
    pub account_hash: String,
}

pub async fn create_tree(Json(payload): Json<CreateTree>) -> (StatusCode, Json<Tree>) {
    // TODO : Create Merkle Root

    // TODO : Save to DB
    let tree = Tree {
        account_hash: payload.account_hash,
        merkle_root: "merkle123".to_string(),
    };

    (StatusCode::CREATED, Json(tree))
}

pub async fn get_tree_messages(
    Path(_): Path<String>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    // TODO : Proof Validation
    let is_valid_proof = true;

    // TODO : Check if after target date
    let is_after_target_date = true;

    if is_valid_proof {

        let messages = vec![
            Message {
                ornament_id: 1,
                nickname: "Alice".to_string(),
                friendship_proof: "proof123".to_string(),
                merkle_root: "root123".to_string(),
                body: Some("Happy Holidays!".to_string()),
            },
        ];

        if is_after_target_date {
            // With Body
            return Ok(Json(messages))
        }

        // Without Body
        Ok(Json(messages))

    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

#[derive(Deserialize)]
pub struct CreateMessage {
    pub nickname: String,
    pub body: String,
    pub friendship_proof: String,
    pub merkle_root: String,
}

pub async fn create_tree_message(Json(payload): Json<CreateMessage>) -> (StatusCode, Json<Message>) {
    // TODO : Save to DB
    let message = Message {
        ornament_id: 1,
        nickname: payload.nickname,
        friendship_proof: payload.friendship_proof,
        merkle_root: payload.merkle_root,
        body: Some(payload.body),
    };

    (StatusCode::CREATED, Json(message))
}
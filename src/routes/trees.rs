use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use crate::db::{connection::DbPool, models, queries::{create_tree, get_tree}};
use crate::utils::proof::decode_proof;
use crate::utils::hash;
use rs_merkle::{MerkleTree, algorithms::Sha256 as Sha256Algorithm};
use sha2::{Sha256, Digest};
use regex::Regex;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTree {
    pub account_proof: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeResponse {
    pub account_id: String,
    pub account_hash: String,
    pub merkle_root: String,
}

pub async fn create_tree_route(
        State(pool): State<Arc<DbPool>>,
        Json(payload): Json<CreateTree>
    ) -> Result<Json<TreeResponse>, StatusCode> {

    // Decode Proof
    let decoded_proof = match decode_proof(&payload.account_proof) {
        Ok(proof) => proof,
        Err(status) => return Err(status),
    };

    // Parse recv data to get screen_name
    let re = Regex::new(r#""screen_name":"([^"]+)""#).unwrap();
    let caps = match re.captures(&decoded_proof) {
        Some(caps) => caps,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let screen_name = match caps.get(1) {
        Some(screen_name) => screen_name.as_str().to_string(),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    println!("Screen Name: {:?}", screen_name);

    // Get Existing Tree
    let conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> = &mut pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tree = match get_tree(conn, &screen_name) {
        Ok(tree) => Some(tree),
        Err(diesel::result::Error::NotFound) => None,
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    if tree.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // Hash Account Proof
    let mut hasher = Sha256::new();
    hasher.update(&screen_name);
    let account_hash: String = format!("{:X}", hasher.finalize());

    let account_hash_bytes = match hash::string_to_hash(&account_hash) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Create Merkle Tree
    let mut merkle_tree: MerkleTree<Sha256Algorithm> = MerkleTree::<Sha256Algorithm>::new();
    merkle_tree.insert(account_hash_bytes);
    merkle_tree.commit();
    let merkle_root_hex = match merkle_tree.root_hex() {
        Some(root) => root,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // let leaves = merkle_tree.leaves().unwrap().iter().map(|node| hex::encode(node)).collect();
    // let merkle_tree_json = MerkleTreeJson {
    //     nodes: leaves,
    // };

    // Save to Pinata
    // let serialized_tree = match serde_json::to_string(&merkle_tree_json) {
    //     Ok(tree) => tree,
    //     Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    // };

    // let upload_result = match upload_file(serialized_tree, account_hash.to_owned()).await {
    //     Ok(result) => result,
    //     Err(err) => {
    //         println!("{}", err);
    //         return Err(StatusCode::INTERNAL_SERVER_ERROR);
    //     }
    // };

    // Save to DB
    let new_tree = models::NewTree {
        account_id: screen_name,
        account_hash: account_hash,
        merkle_root: merkle_root_hex,
        // proof_file_id: upload_result.data.id,
    };

    let tree = match create_tree(conn, &new_tree) {
        Ok(tree) => tree,
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    // Response
    let tree_res = TreeResponse {
        account_id: tree.account_id,
        account_hash: tree.account_hash,
        merkle_root: tree.merkle_root,
    };
    
    Ok(Json(tree_res))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTreeMessages {
    pub account_proof: String,
}

pub async fn get_tree_messages_route(
    Path(account_hash): Path<String>
) -> Result<Json<Vec<Message>>, StatusCode> {

        let messages = vec![
            Message {
                ornament_id: 1,
                nickname: "Alice".to_string(),
                friendship_proof: "proof123".to_string(),
                merkle_root: "root123".to_string(),
                body: Some("Happy Holidays!".to_string()),
            },
        ];

        // Without Body
        Ok(Json(messages))

}

#[derive(Deserialize)]
pub struct CreateMessage {
    pub nickname: String,
    pub body: String,
    pub friendship_proof: String,
    pub merkle_root: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub ornament_id: u32,
    pub nickname: String,
    pub friendship_proof: String,
    pub merkle_root: String,
    pub body: Option<String>,  // Shown after duedate
}

pub async fn create_tree_message_route(Json(payload): Json<CreateMessage>) -> (StatusCode, Json<Message>) {
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
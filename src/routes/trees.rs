use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;
use tlsn_core::presentation::{Presentation, PresentationOutput};
use tlsn_core::CryptoProvider;
use crate::models::{Message, Tree};
use crate::utils::hash;
use rs_merkle::{MerkleTree, algorithms::Sha256 as Sha256Algorithm};
use sha2::{Sha256, Digest};
use regex::Regex;

#[derive(Deserialize)]
pub struct CreateTree {
    pub account_proof: String,
}

pub async fn create_tree(Json(payload): Json<CreateTree>) -> Result<Json<Tree>, StatusCode> {
    // Decode Proof
    let decoded_proof = match hex::decode(&payload.account_proof) {
        Ok(proof) => proof,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let presentation: Presentation = match bincode::deserialize(&decoded_proof) {
        Ok(presentation) => presentation,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let provider = CryptoProvider::default();

    // Verify the presentation.
    let PresentationOutput {
        transcript,
        ..
    } = presentation.verify(&provider).unwrap();

    // The time at which the connection was started.
    let mut partial_transcript = transcript.unwrap();
    // Set the unauthenticated bytes so they are distinguishable.
    partial_transcript.set_unauthed(b'X');

    let recv = String::from_utf8_lossy(partial_transcript.received_unsafe());

    // Parse recv data to get screen_name
    let re = Regex::new(r#""screen_name":"([^"]+)""#).unwrap();
    let caps = match re.captures(&recv) {
        Some(caps) => caps,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let screen_name = match caps.get(1) {
        Some(screen_name) => screen_name.as_str().to_string(),
        None => return Err(StatusCode::BAD_REQUEST),
    };

    println!("Screen Name: {:?}", screen_name);

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

    // TODO : Save to DB
    let tree = Tree {
        account_id: screen_name,
        account_hash,
        merkle_root: merkle_root_hex,
    };

    Ok(Json(tree))
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
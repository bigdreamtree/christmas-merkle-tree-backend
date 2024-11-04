use axum::{extract::{Path, State}, http::StatusCode, Json};
use regex::Regex;
use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};
use serde::{Serialize, Deserialize};
use crate::{db::{connection::DbPool, models::NewMessage, queries::{create_message, get_messages, get_tree, update_tree_merkle_root}}, utils::{hash::string_to_hash_bytes, pinata::upload_file, proof::{decode_proof, ProofJson}}};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub hash: String,
    pub ornament_id: i32,
    pub nickname: String,
    pub merkle_root: String,
    pub merkle_idx: i32,
    pub merkle_proof: String,
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
            merkle_idx: message.merkle_idx,
            merkle_proof: message.merkle_proof.clone(),
            body: None,
        }
    }).collect();

    Ok(Json(response))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevealTreeMessages {
    pub account_proof: ProofJson,
}

pub async fn get_tree_messages_reveal_route(
    State(pool): State<Arc<DbPool>>,
    Path(account_hash): Path<String>,
    Json(payload): Json<RevealTreeMessages>
) -> Result<Json<Vec<MessageResponse>>, StatusCode> {

    // Check Proof
    let decoded_proof = match decode_proof(&payload.account_proof.data) {
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

    // Hash Account Proof
    let parsed_account_hash: String = hex::encode(Sha256::hash(screen_name.as_bytes()));

    if account_hash != parsed_account_hash {
        println!("Account Hash Mismatch {:?} != {:?}", parsed_account_hash, account_hash);
        return Err(StatusCode::BAD_REQUEST);
    }

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
            merkle_idx: message.merkle_idx,
            merkle_proof: message.merkle_proof.clone(),
            body: Some(message.body.clone()),
        }
    }).collect();

    Ok(Json(response))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessage {
    pub ornament_id: i32,
    pub nickname: String,
    pub body: String,
    pub friendship_proof: ProofJson,
}

pub async fn create_tree_message_route(
    State(pool): State<Arc<DbPool>>,
    Path(account_hash): Path<String>,
    Json(payload): Json<CreateMessage>
) -> Result<Json<MessageResponse>, StatusCode> {

    // Decode Proof
    println!("Decoding Proof");
    let decoded_proof = match decode_proof(&payload.friendship_proof.data) {
        Ok(proof) => proof,
        Err(status) => return Err(status),
    };

    // Parse recv data to check if following each other
    let re = Regex::new(r#""followed_by":\s*true\s*,\s*"following":\s*true"#).unwrap();
    if re.is_match(&decoded_proof) == false {
        return Err(StatusCode::BAD_REQUEST);
    }

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

    // Hash Account Proof
    let account_hash_bytes = Sha256::hash(screen_name.as_bytes());
    let account_hash_from_twt: String = hex::encode(account_hash_bytes);
    if account_hash_from_twt != account_hash {
        println!("Account Hash Mismatch {:?} != {:?}", account_hash_from_twt, account_hash);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Get Existing Tree
    println!("Loading mekle tree");
    let conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> = &mut pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tree = match get_tree(conn, &account_hash) {
        Ok(tree) => tree,
        Err(diesel::result::Error::NotFound) => {
            println!("Tree Not Found");
            return Err(StatusCode::NOT_FOUND);
        },
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    // Get Tree Leaves
    let messages = match get_messages(conn, &account_hash) {
        Ok(messages) => messages,
        Err(diesel::result::Error::NotFound) => vec![],
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };
    let mut leaves: Vec<[u8; 32]> = messages.iter().map(|message| string_to_hash_bytes(&message.hash).unwrap()).collect();
    leaves.insert(0, account_hash_bytes);
    
    // Load Merkle Tree
    println!("Loading merkle tree from leaves");
    let mut merkle_tree: MerkleTree<Sha256> = MerkleTree::<Sha256>::from_leaves(&leaves);

    let new_leaves: Vec<String> = merkle_tree.leaves().unwrap().iter().map(|node| hex::encode(node)).collect();
    println!("Leaves: {:?}", new_leaves);

    // Check merkle is valid
    let initial_merkle_root_hex = merkle_tree.root_hex().unwrap();
    if tree.merkle_root != initial_merkle_root_hex {
        println!("Merkle Root Mismatch {:?} != {:?}", tree.merkle_root, initial_merkle_root_hex);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Prepare new message
    let message_idx = messages.len();
    let proof_hash = Sha256::hash(payload.friendship_proof.data.as_bytes());
    let body_hash = Sha256::hash(payload.body.as_bytes());
    let message_hash = Sha256::hash(&[proof_hash, body_hash].concat());
    let message_hash_hex = hex::encode(message_hash);

    // Check if message already exists
    for message in &messages {
        if message.hash == message_hash_hex {
            return Err(StatusCode::CONFLICT);
        }
    }

    // Insert new message to Merkle Tree
    merkle_tree.insert(message_hash);
    merkle_tree.commit();

    let merkle_root = match merkle_tree.root() {
        Some(root) => root,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Merkle Proof Verification
    println!("Verifying merkle proof");
    let indices_to_prove = [message_idx];
    let leaves_to_prove = [message_hash];
    let merkle_proof =  merkle_tree.proof(&indices_to_prove);
    let proof_bytes = merkle_proof.to_bytes();
    let proof: MerkleProof<Sha256> = MerkleProof::from_bytes(&proof_bytes).unwrap();

    let result = proof.verify(merkle_root, &indices_to_prove, &leaves_to_prove, merkle_tree.leaves_len());
    print!("Merkle Proof Verification Result: {:?}", result);

    // Save Proof to Pinata
    let serialized_proof = match serde_json::to_string(&payload.friendship_proof) {
        Ok(tree) => tree,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let upload_result = match upload_file(serialized_proof, message_hash_hex.to_owned()).await {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Save to DB
    let new_message = NewMessage {
        hash: message_hash_hex,
        merkle_idx: messages.len() as i32,
        merkle_proof: hex::encode(proof_bytes),
        parent_account_hash: account_hash,
        ornament_id: payload.ornament_id,
        nickname: payload.nickname,
        body: payload.body,
        proof_file_id: upload_result.data.id,
    };

    let message = match create_message(conn, &new_message) {
        Ok(message) => message,
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
    };

    let new_merkle_root_hex = hex::encode(merkle_root);
    println!("Updating tree merkle root to {:?}", new_merkle_root_hex);

    if update_tree_merkle_root(conn, &tree, new_merkle_root_hex.to_owned()) != Ok(1) {
        println!("Failed to update tree merkle root");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let response = MessageResponse {
        hash: message.hash,
        merkle_idx: message.merkle_idx,
        merkle_proof: message.merkle_proof,
        ornament_id: message.ornament_id,
        nickname: message.nickname,
        merkle_root: new_merkle_root_hex,
        body: Some(message.body),
    };

    Ok(Json(response))
}
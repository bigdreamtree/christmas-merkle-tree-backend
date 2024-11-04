use axum::{extract::State, http::StatusCode, Json};
use serde::{Serialize, Deserialize};
use crate::{db::{connection::DbPool, models, queries::{create_tree, get_tree}}, utils::{pinata::upload_file, proof::ProofJson}};
use crate::utils::proof::decode_proof;
use crate::utils::hash;
use rs_merkle::{MerkleTree, algorithms::Sha256 as Sha256Algorithm};
use sha2::{Sha256, Digest};
use regex::Regex;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTree {
    pub account_proof: ProofJson,
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

    println!("Screen Name: {:?}", screen_name);

    // Hash Account Proof
    let mut hasher = Sha256::new();
    hasher.update(&screen_name);
    let account_hash: String = format!("{:X}", hasher.finalize());

    let account_hash_bytes = match hash::string_to_hash(&account_hash) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Get Existing Tree
    let conn: &mut diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> = &mut pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let tree = match get_tree(conn, &account_hash) {
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

    // Create Merkle Tree
    let mut merkle_tree: MerkleTree<Sha256Algorithm> = MerkleTree::<Sha256Algorithm>::new();
    merkle_tree.insert(account_hash_bytes);
    merkle_tree.commit();
    let merkle_root_hex = match merkle_tree.root_hex() {
        Some(root) => root,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Save Proof to Pinata
    let serialized_proof = match serde_json::to_string(&payload.account_proof) {
        Ok(tree) => tree,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let upload_result = match upload_file(serialized_proof, account_hash.to_owned()).await {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Save to DB
    let new_tree = models::NewTree {
        account_id: screen_name,
        account_hash: account_hash,
        merkle_root: merkle_root_hex,
        proof_file_id: upload_result.data.id,
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
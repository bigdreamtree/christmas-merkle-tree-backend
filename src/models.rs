use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub ornament_id: u32,
    pub nickname: String,
    pub friendship_proof: String,
    pub merkle_root: String,
    pub body: Option<String>,  // Shown after duedate
}

#[derive(Serialize, Deserialize)]
pub struct Tree {
    pub account_id: String,
    pub account_hash: String,
    pub merkle_root: String,
}
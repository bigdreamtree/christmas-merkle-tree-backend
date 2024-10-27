use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub ornament_id: u32,
    pub nickname: String,
    pub friendship_proof: String,
    pub merkle_root: String,
    pub body: Option<String>,  // Shown after duedate
}
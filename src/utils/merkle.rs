use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MerkleTreeJson {
    pub nodes: Vec<String>,
}
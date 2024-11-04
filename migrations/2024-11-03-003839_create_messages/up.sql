CREATE TABLE messages (
    hash VARCHAR(64) NOT NULL PRIMARY KEY,
    merkle_idx INTEGER NOT NULL,
    merkle_proof VARCHAR(255) NOT NULL,
    parent_account_hash VARCHAR(64) NOT NULL,
    ornament_id INTEGER NOT NULL,
    nickname VARCHAR(255) NOT NULL,
    proof_file_id VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)
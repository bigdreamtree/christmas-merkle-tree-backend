CREATE TABLE trees (
    account_hash VARCHAR(64) NOT NULL PRIMARY KEY,
    account_id VARCHAR(255) NOT NULL,
    merkle_root VARCHAR(64) NOT NULL,
    proof_file_id VARCHAR(255) NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)
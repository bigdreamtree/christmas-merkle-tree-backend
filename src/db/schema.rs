diesel::table! {
    trees (id) {
        id -> Integer,
        account_id -> VarChar,
        account_hash -> VarChar,
        merkle_root -> VarChar,
        proof_file_id -> VarChar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Integer,
        ornament_id -> VarChar,
        nickname -> VarChar,
        proof_file_id -> VarChar,
        created_at -> Timestamp,
    }
}
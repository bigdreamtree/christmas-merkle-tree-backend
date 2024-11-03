// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Integer,
        ornament_id -> Integer,
        nickname -> Text,
        proof_file_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    trees (id) {
        id -> Integer,
        account_id -> Text,
        account_hash -> Text,
        merkle_root -> Text,
        proof_file_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    trees,
);

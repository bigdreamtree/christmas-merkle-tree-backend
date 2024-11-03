// @generated automatically by Diesel CLI.

diesel::table! {
    messages (hash) {
        hash -> Text,
        parent_account_hash -> Text,
        ornament_id -> Integer,
        nickname -> Text,
        proof_file_id -> Text,
        body -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    trees (account_hash) {
        account_hash -> Text,
        account_id -> Text,
        merkle_root -> Text,
        proof_file_id -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    trees,
);

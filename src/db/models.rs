use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Identifiable};
use diesel::prelude::{Insertable, Queryable, Selectable};
use crate::db::schema::trees;
use crate::db::schema::messages;

#[derive(Debug, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = trees, primary_key(account_hash))]
pub struct Tree {
    pub account_hash: String,
    pub account_id: String,
    pub merkle_root: String,
    pub proof_file_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = trees)]
pub struct NewTree {
    pub account_hash: String,
    pub account_id: String,
    pub merkle_root: String,
    pub proof_file_id: String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = messages, primary_key(hash))]
pub struct Message {
    pub hash: String,
    pub merkle_idx: i32,
    pub merkle_proof: String,
    // pub parent_account_hash: String,
    pub ornament_id: i32,
    pub nickname: String,
    pub body: String,
    // pub proof_file_id: String,
    // pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub hash: String,
    pub merkle_idx: i32,
    pub merkle_proof: String,
    pub parent_account_hash: String,
    pub ornament_id: i32,
    pub nickname: String,
    pub body: String,
    pub proof_file_id: String,
}
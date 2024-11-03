use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Selectable};
use crate::db::schema::trees;
use crate::db::schema::messages;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = trees)]
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
#[diesel(table_name = messages)]
pub struct Message {
    pub hash: String,
    pub parent_account_hash: String,
    pub ornament_id: i32,
    pub nickname: String,
    pub proof_file_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub hash: String,
    pub parent_account_hash: String,
    pub ornament_id: i32,
    pub nickname: String,
    pub proof_file_id: String,
}
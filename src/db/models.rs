use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Selectable};
use crate::db::schema::trees;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = trees)]
pub struct Tree {
    pub id: i32,
    pub account_id: String,
    pub account_hash: String,
    pub merkle_root: String,
    pub proof_file_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = trees)]
pub struct NewTree {
    pub account_id: String,
    pub account_hash: String,
    pub merkle_root: String,
    // pub proof_file_id: String,
}
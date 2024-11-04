use diesel::prelude::*;
use crate::db::models::Tree;
use crate::db::models::NewTree;
use crate::db::models::Message;
use crate::db::models::NewMessage;

pub fn get_tree(conn: &mut SqliteConnection, account_hash_str: &str) -> Result<Tree, diesel::result::Error> {
    use crate::db::schema::trees::dsl::*;

    trees
        .filter(account_hash.eq(account_hash_str))
        .select(Tree::as_select())
        .first(conn)
}

pub fn create_tree(conn: &mut SqliteConnection, new_tree: &NewTree) -> Result<Tree, diesel::result::Error> {
    use crate::db::schema::trees;

    diesel::insert_into(trees::table)
        .values(new_tree)
        .returning(Tree::as_returning())
        .get_result(conn)
}

pub fn update_tree_merkle_root(conn: &mut SqliteConnection, tree: &Tree, merkle_root: String) -> Result<usize, diesel::result::Error> {
    use crate::db::schema::trees;

    diesel::update(tree)
        .set(trees::merkle_root.eq(merkle_root))
        .execute(conn)
}

pub fn get_messages(conn: &mut SqliteConnection, account_hash_str: &str) -> Result<Vec<Message>, diesel::result::Error> {
    use crate::db::schema::messages::dsl::*;

    messages
        .filter(parent_account_hash.eq(account_hash_str))
        .select(Message::as_select())
        .load(conn)
}

pub fn create_message(conn: &mut SqliteConnection, new_message: &NewMessage) -> Result<Message, diesel::result::Error> {
    use crate::db::schema::messages;

    diesel::insert_into(messages::table)
        .values(new_message)
        .returning(Message::as_returning())
        .get_result(conn)
}
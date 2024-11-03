use diesel::prelude::*;
use crate::db::models::Tree;
use crate::db::models::NewTree;

pub fn get_tree(conn: &mut SqliteConnection, account_id_str: &str) -> Result<Tree, diesel::result::Error> {
    use crate::db::schema::trees::dsl::*;

    let tree = trees
        .filter(account_id.eq(account_id_str))
        .select(Tree::as_select())
        .first(conn);

    tree
}

pub fn create_tree(conn: &mut SqliteConnection, new_tree: &NewTree) -> Result<Tree, diesel::result::Error> {
    use crate::db::schema::trees;

    let result = diesel::insert_into(trees::table)
        .values(new_tree)
        .returning(Tree::as_returning())
        .get_result(conn);

    result
}
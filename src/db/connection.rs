use std::env;
use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn create_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DynError = Box<dyn std::error::Error>;

pub fn get_db_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(format!("Error connecting to {}", &database_url).as_str());

    pool.get()
        .expect("couldn't get conn to enable foreign keys")
        .execute("PRAGMA foreign_keys = ON;")
        .expect("couldn't enable foreign keys");

    return pool;
}

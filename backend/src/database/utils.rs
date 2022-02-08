use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::embed_migrations;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DynError = Box<dyn std::error::Error>;

embed_migrations!("migrations");

pub fn get_db_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(format!("Error connecting to {}", &database_url).as_str());

    let conn = pool
        .get()
        .expect("couldn't get conn to perform preparational tasks");

    conn.execute("PRAGMA foreign_keys = ON;")
        .expect("couldn't enable foreign keys");

    embedded_migrations::run(&conn).expect("Could not run migrations!");

    return pool;
}

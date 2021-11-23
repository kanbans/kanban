use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_db_pool() -> Pool {
	let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = 
        ConnectionManager::<SqliteConnection>::new(&database_url);

    r2d2::Pool::builder().build(manager)
		.expect(format!("Error connecting to {}", &database_url).as_str())
}

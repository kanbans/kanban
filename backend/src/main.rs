pub mod database;
pub mod routes;
pub mod utils;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use database::utils::get_db_pool;
use dotenv::dotenv;
use slog::info;
use std::env;
use utils::misc::get_logger;
use utils::models::State;

#[macro_use]
extern crate diesel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());

    let log = get_logger();

    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT should be a valid number");

    info!(log, "Starting kanban server on {}:{}", host, port);

    info!(log, "WARN: using permissive CORS");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .data(State {
                pool: get_db_pool(),
                log: log.clone(),
            })
            // root routes (/)
            .service(routes::root::root)
            // user routes (/user)
            .service(routes::user::register)
            .service(routes::user::login)
            .service(routes::user::login)
            .service(routes::user::me)
            // board routes (/board)
            .service(routes::board::read)
    })
    .bind((host, port))?
    .run()
    .await
}

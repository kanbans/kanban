pub mod database;
pub mod routes;
pub mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use database::utils::get_db_pool;
use dotenv::dotenv;
use slog::info;
use std::env;
use utils::misc::get_logger;
use utils::models::State;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

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
            .service(routes::user::me)
            // board routes (/board)
            .service(routes::board::create)
            .service(routes::board::read)
            .service(routes::board::update)
            .service(routes::board::delete)
            // card routes (/card)
            .service(routes::card::create)
            .service(routes::card::read)
            .service(routes::card::update)
            .service(routes::card::delete)
            // column routes (/column)
            .service(routes::column::create)
            .service(routes::column::read)
            .service(routes::column::update)
            .service(routes::column::delete)
    })
    .bind((host, port))?
    .run()
    .await
}

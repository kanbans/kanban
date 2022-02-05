use crate::utils::misc::from_blocking_err;
use crate::utils::models::AppError;
use crate::{database::entities::board, utils::models::State};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use serde_json::json;

#[post("/board")]
async fn create(
    state: web::Data<State>,
    req: HttpRequest,
    name: String,
) -> Result<HttpResponse, AppError> {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        board::utils::create_board(&conn, &name)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[get("/board")]
async fn read(state: web::Data<State>, req: HttpRequest) -> Result<HttpResponse, AppError> {
    let log = &state.log.clone();

    let boards = web::block(move || {
        let conn = state.pool.get()?;
        board::utils::get_boards(&conn)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "boards": boards
    })))
}

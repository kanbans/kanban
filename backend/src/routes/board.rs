use crate::utils::misc::from_blocking_err;
use crate::utils::models::AppError;
use crate::{database::entities::board, utils::models::State};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde_json::json;

type Resp = Result<HttpResponse, AppError>;

#[post("/board")]
async fn create(state: web::Data<State>, req: HttpRequest, name: String) -> Resp {
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

#[put("/board")]
async fn update(state: web::Data<State>, req: HttpRequest, cid: String, new_name: String) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        board::utils::update_name(&conn, cid, new_name)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[delete("/board")]
async fn delete(state: web::Data<State>, req: HttpRequest, cid: String) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        board::utils::delete(&conn, cid)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[get("/board")]
async fn read(state: web::Data<State>, req: HttpRequest) -> Resp {
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

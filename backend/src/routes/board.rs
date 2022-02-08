use crate::database::entities::user::model::User;
use crate::utils::misc::from_blocking_err;
use crate::utils::models::AppError;
use crate::{database::entities::board, utils::models::State};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde_json::json;

type Resp = Result<HttpResponse, AppError>;

#[post("/board")]
async fn create(state: web::Data<State>, req: HttpRequest, name: String, user: User) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        board::utils::create_board(&conn, &name, &user.id)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[put("/board")]
async fn update(
    state: web::Data<State>,
    req: HttpRequest,
    cid: String,
    new_name: String,
    user: User,
) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        let board = board::utils::get_from_id(&conn, &cid)?;
        if board.belongs_to == user.id {
            board::utils::update_name(&conn, &cid, &new_name)
        } else {
            Err(Box::new(AppError::Forbidden))
        }
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
        board::utils::delete(&conn, &cid)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[get("/board")]
async fn read(state: web::Data<State>, req: HttpRequest, user: User) -> Resp {
    let log = &state.log.clone();

    let boards = web::block(move || {
        let conn = state.pool.get()?;
        board::utils::get_boards(&conn, &user.id)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "boards": boards
    })))
}

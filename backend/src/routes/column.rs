use crate::database::entities::board;
use crate::database::entities::{column, user::model::User};
use actix_web::{delete, get, put, web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::utils::{
    misc::from_blocking_err,
    models::{AppError, State},
};

type Resp = Result<HttpResponse, AppError>;

#[get("/column")]
async fn create(
    state: web::Data<State>,
    req: HttpRequest,
    user: User,
    name: String,
    belongs_to: String,
) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;

        let board = board::utils::get_from_id(&conn, &belongs_to)?;
        if board.belongs_to != user.id {
            Err(AppError::Forbidden)?;
        }

        column::utils::create_column(&conn, &name, &belongs_to)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[put("/column")]
async fn update(
    state: web::Data<State>,
    req: HttpRequest,
    cid: String,
    new_name: Option<String>,
    new_belongs_to: Option<String>,
    user: User,
) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;

        if let Some(v) = new_belongs_to.clone() {
            let board = board::utils::get_from_id(&conn, &v)?;
            if board.belongs_to != user.id {
                Err(AppError::Forbidden)?;
            }
        }

        column::utils::update_column(&conn, &cid, &new_name, &new_belongs_to)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[get("/column")]
async fn read(state: web::Data<State>, req: HttpRequest, board_id: String, user: User) -> Resp {
    let log = &state.log.clone();

    let columns = web::block(move || {
        let conn = state.pool.get()?;

        let board = board::utils::get_from_id(&conn, &board_id)?;
        if board.belongs_to != user.id {
            Err(AppError::Forbidden)?;
        }

        column::utils::get_columns(&conn, &board_id)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "columns": columns
    })))
}

#[delete("/column")]
async fn delete(state: web::Data<State>, req: HttpRequest, user: User, cid: String) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;

        let existing = column::utils::get_from_id(&conn, &cid)?;
        let board = board::utils::get_from_id(&conn, &existing.belongs_to)?;
        if board.belongs_to != user.id {
            Err(AppError::Forbidden)?;
        }

        column::utils::delete_column(&conn, &cid)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

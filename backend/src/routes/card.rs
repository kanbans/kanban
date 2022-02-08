use crate::database::entities::{card, user::model::User};
use actix_web::{get, put, web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::utils::{
    misc::from_blocking_err,
    models::{AppError, State},
};

type Resp = Result<HttpResponse, AppError>;

#[get("/card")]
async fn create(
    state: web::Data<State>,
    req: HttpRequest,
    codename: String,
    title: String,
    description: String,
    priority: String,
    column: String,
    assigned_to: Option<String>,
    user: User,
) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        let priority: i32 = priority.parse()?;

        card::utils::create_card(
            &conn,
            &codename,
            &title,
            &description,
            &priority,
            &column,
            &user.id,
            assigned_to,
        )
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

#[put("/card")]
async fn update(
    state: web::Data<State>,
    req: HttpRequest,
    cid: String,
    codename: Option<String>,
    title: Option<String>,
    description: Option<String>,
    priority: Option<String>,
    column: Option<String>,
    assigned_to: Option<String>,
    user: User,
) -> Resp {
    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        let priority = priority
            .map(|v| v.parse::<i32>())
            .map_or(Ok(None), |v| v.map(Some))?;

        let existing = card::utils::get_from_id(&conn, &cid)?;
        if existing.created_by != user.id {
            Err(AppError::Forbidden)?;
        }

        card::utils::update_card(
            &conn,
            &cid,
            &codename,
            &title,
            &description,
            &priority,
            &column,
            &assigned_to,
        )
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
    })))
}

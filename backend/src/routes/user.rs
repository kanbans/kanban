use crate::database::entities;
use crate::database::entities::user::model::User;
use crate::utils::misc::from_blocking_err;
use crate::utils::models::AppError;
use crate::utils::models::State;
use actix_web::HttpRequest;
use actix_web::{post, web, HttpResponse};
use libreauth::key::KeyBuilder;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use serde::Deserialize;
use serde_json::json;
use slog::error;

#[derive(Deserialize)]
struct RegisterP {
    name: String,
    email: String,
    password: String,
}

#[post("/user/register")]
async fn register(
    state: web::Data<State>,
    body: web::Json<RegisterP>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let session_token = KeyBuilder::new().size(64).as_base64();
    let token = session_token.clone();

    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2
            .hash_password(body.password.as_bytes(), &salt)
            .map_err(|_| AppError::Unknown)?
            .to_string();

        let user =
            entities::user::utils::create_user(&conn, &body.name, &body.email, &password_hash)?;

        entities::session::utils::create_session(&conn, &token, &user.id)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "session_token": session_token,
    })))
}

#[derive(Deserialize)]
struct LoginP {
    email: String,
    password: String,
}

#[post("/user/login")]
async fn login(
    state: web::Data<State>,
    body: web::Json<LoginP>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let session_token = KeyBuilder::new().size(64).as_base64();
    let token = session_token.clone();

    let log = &state.log.clone();

    web::block(move || {
        let conn = state.pool.get()?;
        let user = entities::user::utils::find_user(&conn, &body.email)
            .map_err(|_| AppError::IncorrectCreds)?;

        let parsed_hash = PasswordHash::new(&user.password).map_err(|e| {
            error!(state.log, "parsed_hash err: {}", e);
            AppError::Unknown
        })?;

        Pbkdf2
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::IncorrectCreds)?;

        entities::session::utils::create_session(&conn, &token, &user.id)
    })
    .await
    .map_err(|e| from_blocking_err(e, log, req))?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "session_token": session_token,
    })))
}

#[post("/user/me")]
async fn me(user: User) -> Result<HttpResponse, AppError> {
    Ok(web::HttpResponse::Ok()
        .json(json!({ "id": user.id, "name": user.name, "email": user.email })))
}

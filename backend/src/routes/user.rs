use crate::database::entities;
use crate::utils::models::AppError;
use crate::utils::models::State;
use actix_web::{post, web, HttpResponse};
use libreauth::key::KeyBuilder;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct RegisterP {
    name: String,
    email: String,
    password: String,
}

#[post("/register")]
async fn register(
    state: web::Data<State>,
    body: web::Json<RegisterP>,
) -> Result<HttpResponse, AppError> {
    let session_token = KeyBuilder::new().size(64).as_base64();
    let token = session_token.clone();

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
    .map_err(|_| AppError::Unknown)?;

    Ok(web::HttpResponse::Ok().json(json!({
        "success": true,
        "session_token": session_token,
    })))
}

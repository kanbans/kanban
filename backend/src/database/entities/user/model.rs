use crate::database::entities::session::utils::find_user_with_session;
use crate::database::schema::users;
use crate::utils::models::AppError;
use crate::utils::models::State;
use actix_http::error::BlockingError;
use actix_http::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use chrono::NaiveDateTime;
use futures::Future;
use std::pin::Pin;

#[derive(Queryable, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

fn get_auth_header<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorisation")?.to_str().ok()
}

impl FromRequest for User {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let r = req.clone();
        let authorization = r.headers().get("Authorization").clone();

        Box::pin(async move {
            let sess_token = authorization
                .ok_or(AppError::NotLoggedIn)?
                .to_str()
                .map_err(|_| AppError::Unknown)?;

            let conn = req
                .extensions()
                .get::<State>()
                .expect("connection not passed as data!")
                .pool
                .get()
                .map_err(|_| AppError::Unknown)?;

            let user: Result<User, BlockingError<AppError>> = web::block(move || {
                find_user_with_session(&conn, &sess_token).map_err(|_| AppError::InvalidSession)
            })
            .await;

            match user {
                Ok(u) => Ok(u),
                Err(_) => Err(AppError::Unknown),
            }
        })
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: String,
    pub name: &'a String,
    pub email: &'a String,
    pub password: &'a String,
}

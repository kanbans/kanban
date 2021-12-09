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

impl FromRequest for User {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, AppError>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let r = req.clone();
        let authorization = r.headers().get("Authorization").cloned();

        let conn = req
            .app_data::<web::Data<State>>()
            .and_then(|i| i.pool.get().ok())
            .ok_or(AppError::Unknown);

        let auth_head = authorization.ok_or(AppError::NotLoggedIn);

        Box::pin(async move {
            let sess_token = auth_head?
                .to_str()
                .map_err(|_| AppError::Unknown)?
                .to_string();

            let conn = conn?;

            let user: Result<User, BlockingError<AppError>> = web::block(move || {
                find_user_with_session(&conn, &sess_token).map_err(|_| AppError::InvalidSession)
            })
            .await;

            match user {
                Ok(u) => Ok(u),
                Err(BlockingError::Canceled) => Err(AppError::Unknown),
                Err(BlockingError::Error(e)) => Err(e),
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

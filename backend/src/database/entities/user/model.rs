use crate::database::schema::users;
use crate::utils::models::AppError;
use actix_http::Payload;
use actix_web::{FromRequest, HttpRequest};
use chrono::NaiveDateTime;
use futures::future::{err, ok};

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
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<User>() {
            Some(user) => return ok(user.clone()),
            None => return err(AppError::Unknown),
        };
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

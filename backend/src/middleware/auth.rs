use crate::database::entities::{session::utils::find_user_with_session, user::model::User};
use crate::utils::models::State;
use actix_http::error::BlockingError;
use actix_http::HttpMessage;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, Error};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::utils::models::AppError;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: Clone,
    S: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = AppError;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: Clone,
    S: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(|_| AppError::Unknown)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let sess_token = req
                .headers()
                .get("Authorisation")
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
                Ok(u) => {
                    req.extensions_mut().insert(u);

                    let res = srv.call(req).await.map_err(|_| AppError::Unknown)?;
                    Ok(res)
                }

                Err(_) => Err(AppError::Unknown),
            }
        })
    }
}

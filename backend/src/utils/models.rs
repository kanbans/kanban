use crate::database::utils::Pool;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use slog;
use thiserror::Error;

#[derive(Clone)]
pub struct State {
    pub pool: Pool,
    pub log: slog::Logger,
}

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Something went wrong -- please try again later")]
    Unknown,
    #[error("Entered username or password is invalid")]
    IncorrectCreds,
    #[error("Your session is invalid, please log in again")]
    InvalidSession,
    #[error("You must log in to perform this action")]
    NotLoggedIn,
}

#[derive(Serialize)]
pub struct ErrResp {
    pub success: bool,
    pub code: u16,
    pub message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IncorrectCreds => StatusCode::UNAUTHORIZED,
            Self::InvalidSession => StatusCode::UNAUTHORIZED,
            Self::NotLoggedIn => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();

        let error_response = ErrResp {
            success: false,
            code: status_code.as_u16(),
            message: self.to_string(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}

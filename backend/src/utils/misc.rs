use actix_http::error::BlockingError;
use actix_web::HttpRequest;
use slog::error;
use slog::{o, Drain, Logger};
use slog_async;
use slog_term;

use crate::database::utils::DbError;

use super::models::{AppError, State};

pub fn get_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}

pub fn from_blocking_err(e: BlockingError<DbError>, state: &State, req: HttpRequest) -> AppError {
    match e {
        BlockingError::Canceled => AppError::Unknown,
        BlockingError::Error(e) => match e.downcast_ref::<AppError>() {
            Some(ae) => ae.clone(),
            None => {
                error!(state.log, "{} -- {}", req.path(), e);
                AppError::Unknown
            }
        },
    }
}

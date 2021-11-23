use slog;
use crate::database::utils::Pool;

#[derive(Clone)]
pub struct State {
    pub pool: Pool,
    pub log: slog::Logger,
}
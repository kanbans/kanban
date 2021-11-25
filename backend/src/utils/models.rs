use crate::database::utils::Pool;
use slog;

#[derive(Clone)]
pub struct State {
    pub pool: Pool,
    pub log: slog::Logger,
}

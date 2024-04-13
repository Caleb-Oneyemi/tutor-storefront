use sqlx::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub visit_count: Mutex<u32>,
    pub db: PgPool,
    pub logger: slog::Logger,
}

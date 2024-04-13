use super::models::Course;
use std::sync::Mutex;

pub struct AppState {
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
    pub logger: slog::Logger,
}

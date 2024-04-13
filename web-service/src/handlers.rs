use super::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use slog::info;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    info!(app_state.logger, "calling health check...");

    let mut visit_count = app_state.visit_count.lock().unwrap();

    *visit_count += 1;

    let response = format!("visit {}: server is up and running", visit_count);

    HttpResponse::Ok().json(response)
}

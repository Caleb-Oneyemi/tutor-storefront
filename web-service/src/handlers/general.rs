use crate::errors::CustomError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use slog::info;

pub async fn health_check(app_state: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    info!(app_state.logger, "calling health check...");

    let mut visit_count = app_state.visit_count.lock().unwrap();

    *visit_count += 1;

    let response = format!("visit {}: server is up and running", visit_count);

    Ok(HttpResponse::Ok().json(response))
}

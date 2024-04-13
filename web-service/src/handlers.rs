use super::state::AppState;
use actix_web::{web, HttpResponse, Responder};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    *visit_count += 1;

    let response = format!("visit {}: {}", visit_count, health_check_response);

    HttpResponse::Ok().json(response)
}

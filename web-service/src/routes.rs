use super::handlers::*;
use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

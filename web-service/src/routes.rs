use super::handlers::{courses, general};
use actix_web::web;

pub fn base_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(general::health_check));
}

pub fn course_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/courses")
            .route("", web::post().to(courses::create))
            .route("", web::get().to(courses::get_all))
            .route("/{id}", web::get().to(courses::get_by_id))
            .route("/{id}", web::put().to(courses::update_by_id))
            .route("/{id}", web::delete().to(courses::delete_one))
            .route(
                "/tutor/{tutor_id}",
                web::get().to(courses::get_all_by_tutor),
            ),
    );
}

use super::handlers::*;
use actix_web::web;

pub fn base_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub fn course_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/courses")
            .route("", web::post().to(create_course))
            .route("", web::get().to(get_all_courses))
            .route("/tutor/{tutor_id}", web::get().to(get_courses_by_tutor)),
    );
}

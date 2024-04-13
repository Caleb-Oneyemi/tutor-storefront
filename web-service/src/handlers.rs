use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use slog::info;
use std::time::SystemTime;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> impl Responder {
    info!(app_state.logger, "calling health check...");

    let mut visit_count = app_state.visit_count.lock().unwrap();

    *visit_count += 1;

    let response = format!("visit {}: server is up and running", visit_count);

    HttpResponse::Ok().json(response)
}

pub async fn create_course_handler(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    info!(
        app_state.logger,
        "calling create course with ---- {:?}", new_course
    );

    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == new_course.tutor_id)
        .count();

    let resp = Course {
        id: Some((course_count + 1).try_into().unwrap()),
        tutor_id: new_course.tutor_id,
        name: new_course.name.clone(),
        created_at: Some(SystemTime::now()),
    };

    app_state.courses.lock().unwrap().push(resp.clone());

    HttpResponse::Created().json(resp)
}

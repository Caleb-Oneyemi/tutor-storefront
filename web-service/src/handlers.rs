use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use slog::info;

use super::data as courses;

pub async fn health_check(app_state: web::Data<AppState>) -> impl Responder {
    info!(app_state.logger, "calling health check...");

    let mut visit_count = app_state.visit_count.lock().unwrap();

    *visit_count += 1;

    let response = format!("visit {}: server is up and running", visit_count);

    HttpResponse::Ok().json(response)
}

pub async fn create_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    info!(
        app_state.logger,
        "calling create course with ---- {:?}", new_course
    );

    let new_course = Course {
        id: None,
        tutor_id: new_course.tutor_id,
        name: new_course.name.clone(),
        created_at: None,
    };

    let res = courses::create(&app_state.db, new_course).await;

    HttpResponse::Created().json(res)
}

pub async fn get_all_courses(app_state: web::Data<AppState>) -> HttpResponse {
    info!(app_state.logger, "calling get all courses...");

    let res = courses::get_all(&app_state.db).await;

    HttpResponse::Ok().json(res)
}

pub async fn get_courses_by_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let tutor_id = path.into_inner();

    info!(
        app_state.logger,
        "calling get course by tutor_id {}...", tutor_id
    );

    let courses = courses::get_by_tutor(&app_state.db, tutor_id).await;

    HttpResponse::Ok().json(courses)
}

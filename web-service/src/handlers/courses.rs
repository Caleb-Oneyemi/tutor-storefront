use super::super::errors::CustomError;
use super::super::models::courses::Course;
use super::super::state::AppState;
use actix_web::{web, HttpResponse};
use slog::info;

use super::super::data::courses;

pub async fn health_check(app_state: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    info!(app_state.logger, "calling health check...");

    let mut visit_count = app_state.visit_count.lock().unwrap();

    *visit_count += 1;

    let response = format!("visit {}: server is up and running", visit_count);

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
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

    courses::create(&app_state.db, new_course)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_all_courses(app_state: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    info!(app_state.logger, "calling get all courses...");

    courses::get_all(&app_state.db)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_courses_by_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let tutor_id = path.into_inner();

    info!(
        app_state.logger,
        "calling get course by tutor_id {}...", tutor_id
    );

    courses::get_by_tutor(&app_state.db, tutor_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_by_course_id(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();

    info!(app_state.logger, "calling get course by id {}...", id);

    courses::get_by_course_id(&app_state.db, id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

use crate::data::courses;
use crate::errors::CustomError;
use crate::models::courses::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use slog::info;

pub async fn create(
    app_state: web::Data<AppState>,
    new_course: web::Json<CreateCourse>,
) -> Result<HttpResponse, CustomError> {
    info!(
        app_state.logger,
        "calling create course with ---- {:?}", new_course
    );

    courses::create(&app_state.db, new_course.into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_all(app_state: web::Data<AppState>) -> Result<HttpResponse, CustomError> {
    info!(app_state.logger, "calling get all courses...");

    courses::get_all(&app_state.db)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_all_by_tutor(
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

pub async fn get_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();

    info!(app_state.logger, "calling get course by id {}...", id);

    courses::get_by_id(&app_state.db, id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn delete_one(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();

    courses::delete_one(&app_state.db, id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_by_id(
    app_state: web::Data<AppState>,
    details: web::Json<UpdateCourse>,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();

    let existing_course = courses::get_by_id(&app_state.db, id).await?;

    courses::update_by_id(&app_state.db, id, existing_course, details.into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

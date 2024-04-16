use actix_web::{web, HttpResponse};
use slog::info;

use crate::{
    data::tutors,
    errors::CustomError,
    models::tutors::{CreateTutor, UpdateTutor},
    state::AppState,
};

pub async fn get_all(app_state: AppState) -> Result<HttpResponse, CustomError> {
    info!(app_state.logger, "calling get all tutors...");

    tutors::get_all(&app_state.db)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn create(
    app_state: AppState,
    data: web::Json<CreateTutor>,
) -> Result<HttpResponse, CustomError> {
    info!(
        app_state.logger,
        "calling create tutors with --- {:?}", data
    );

    tutors::create(&app_state.db, data.into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn get_tutor_by_id(
    app_state: AppState,
    path: web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();

    info!(app_state.logger, "calling get tutor by id {}", id);

    tutors::get_by_id(&app_state.db, id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

pub async fn update_tutor(
    app_state: AppState,
    path: web::Path<i32>,
    data: web::Json<UpdateTutor>,
) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner();

    info!(
        app_state.logger,
        "calling update tutor with id {} and body {:?}", id, data
    );

    let existing_tutor = tutors::get_by_id(&app_state.db, id).await?;

    tutors::update(&app_state.db, id, existing_tutor, data.into())
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

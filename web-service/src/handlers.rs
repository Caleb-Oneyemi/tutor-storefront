use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use slog::info;

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
        created_at: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(resp.clone());

    HttpResponse::Created().json(resp)
}

#[cfg(test)]
mod tests {
    use crate::logger::get_logger;

    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_web::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            name: "rust 101".to_string(),
            id: None,
            created_at: None,
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
            logger: get_logger(),
        });

        let resp = create_course_handler(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}

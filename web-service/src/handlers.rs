use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use slog::info;

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

pub async fn get_all_courses(app_state: web::Data<AppState>) -> HttpResponse {
    info!(app_state.logger, "calling get all courses...");

    let courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .collect::<Vec<Course>>();

    if courses.len() == 0 {
        return HttpResponse::Ok().json(Vec::<Course>::new());
    }

    HttpResponse::Ok().json(courses)
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

    let courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if courses.len() == 0 {
        return HttpResponse::Ok().json(Vec::<Course>::new());
    }

    HttpResponse::Ok().json(courses)
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

        let resp = create_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn get_all_courses_test() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
            logger: get_logger(),
        });

        let resp = get_all_courses(app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_courses_by_tutor_test() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
            logger: get_logger(),
        });

        let path: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_by_tutor(app_state, path).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

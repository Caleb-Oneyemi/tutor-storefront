use super::super::errors::CustomError;
use super::super::models::courses::Course;
use sqlx::postgres::PgPool;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Course>, CustomError> {
    let db_rows = sqlx::query!("SELECT * from tutor_storefront")
        .fetch_all(pool)
        .await?;

    let courses = db_rows
        .iter()
        .map(|c| Course {
            id: Some(c.id),
            tutor_id: c.tutor_id,
            name: c.name.clone(),
            created_at: Some(chrono::NaiveDateTime::from(c.created_at.unwrap())),
        })
        .collect();

    Ok(courses)
}

pub async fn get_by_tutor(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, CustomError> {
    let db_rows = sqlx::query!(
        "SELECT * from tutor_storefront where tutor_id = ($1)",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    let courses = db_rows
        .iter()
        .map(|c| Course {
            id: Some(c.id),
            tutor_id: c.tutor_id,
            name: c.name.clone(),
            created_at: c.created_at,
        })
        .collect();

    Ok(courses)
}

pub async fn create(pool: &PgPool, new_course: Course) -> Result<Course, CustomError> {
    let course = sqlx::query!(
        "INSERT INTO tutor_storefront (tutor_id, name) values ($1, $2) returning id, tutor_id, name, created_at",
        new_course.tutor_id,
        new_course.name
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        id: Some(course.id),
        tutor_id: course.tutor_id,
        name: course.name,
        created_at: course.created_at,
    })
}

pub async fn get_by_course_id(pool: &PgPool, course_id: i32) -> Result<Course, CustomError> {
    let db_row = sqlx::query!("SELECT * FROM tutor_storefront where id = $1", course_id,)
        .fetch_one(pool)
        .await;

    match db_row {
        Err(_) => Err(CustomError::NotFoundError("course not found".to_string())),
        Ok(res) => Ok(Course {
            id: Some(res.id),
            tutor_id: res.tutor_id,
            name: res.name.clone(),
            created_at: Some(res.created_at.unwrap()),
        }),
    }
}

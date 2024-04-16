use super::super::errors::CustomError;
use super::super::models::courses::*;
use sqlx::postgres::PgPool;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Course>, CustomError> {
    let db_rows = sqlx::query_as!(Course, "SELECT * from courses order by id DESC")
        .fetch_all(pool)
        .await?;

    Ok(db_rows)
}

pub async fn get_by_tutor(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, CustomError> {
    let db_rows = sqlx::query_as!(
        Course,
        "SELECT * from courses where tutor_id = ($1)",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    Ok(db_rows)
}

pub async fn create(pool: &PgPool, data: CreateCourse) -> Result<Course, CustomError> {
    let course= sqlx::query_as!(
        Course,
        "INSERT INTO courses (tutor_id, name, description, duration, level, format, language, structure, price) values ($1,$2,$3,$4,$5,$6,$7,$8,$9) 
        returning id, tutor_id, name, description, duration, level, format, language, structure, price, created_at, updated_at", 
        data.tutor_id, data.name, data.description, data.duration, data.level, data.format, data.language, data.structure, data.price,
    )
    .fetch_one(pool)
    .await?;

    Ok(course)
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Course, CustomError> {
    let db_row = sqlx::query_as!(Course, "SELECT * FROM courses where id = $1", id,)
        .fetch_one(pool)
        .await;

    match db_row {
        Err(_) => Err(CustomError::NotFoundError("course not found".to_string())),
        Ok(res) => Ok(res),
    }
}

pub async fn delete_one(pool: &PgPool, id: i32) -> Result<String, CustomError> {
    let db_res = sqlx::query!("DELETE FROM courses where id = $1", id)
        .execute(pool)
        .await?;

    Ok(format!("Deleted {} records", db_res.rows_affected()))
}

use super::super::errors::CustomError;
use super::super::models::courses::*;
use chrono::Utc;
use sqlx::postgres::PgPool;

pub async fn get_all(pool: &PgPool) -> Result<Vec<Course>, CustomError> {
    let db_res = sqlx::query_as!(Course, "SELECT * from courses order by id DESC")
        .fetch_all(pool)
        .await?;

    Ok(db_res)
}

pub async fn get_by_tutor(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, CustomError> {
    let db_res = sqlx::query_as!(
        Course,
        "SELECT * from courses where tutor_id = ($1)",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    Ok(db_res)
}

pub async fn create(pool: &PgPool, data: CreateCourse) -> Result<Course, CustomError> {
    let db_res = sqlx::query_as!(
        Course,
        "INSERT INTO courses (tutor_id, name, description, duration, level, format, language, structure, price) values ($1,$2,$3,$4,$5,$6,$7,$8,$9) 
        returning id, tutor_id, name, description, duration, level, format, language, structure, price, created_at, updated_at", 
        data.tutor_id, data.name, data.description, data.duration, data.level, data.format, data.language, data.structure, data.price,
    )
    .fetch_one(pool)
    .await?;

    Ok(db_res)
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Course, CustomError> {
    let db_res = sqlx::query_as!(Course, "SELECT * FROM courses where id = $1", id,)
        .fetch_one(pool)
        .await;

    match db_res {
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

pub async fn update_by_id(
    pool: &PgPool,
    id: i32,
    existing_course: Course,
    details: UpdateCourse,
) -> Result<Course, CustomError> {
    let name: String = if let Some(name) = details.name {
        name
    } else {
        existing_course.name
    };

    let description: String = if let Some(desc) = details.description {
        desc
    } else {
        existing_course.description.unwrap_or_default()
    };

    let format: String = if let Some(format) = details.format {
        format
    } else {
        existing_course.format.unwrap_or_default()
    };

    let structure: String = if let Some(structure) = details.structure {
        structure
    } else {
        existing_course.structure.unwrap_or_default()
    };

    let duration = if let Some(duration) = details.duration {
        duration
    } else {
        existing_course.duration.unwrap_or_default()
    };

    let level: String = if let Some(level) = details.level {
        level
    } else {
        existing_course.level.unwrap_or_default()
    };

    let language: String = if let Some(language) = details.language {
        language
    } else {
        existing_course.language.unwrap_or_default()
    };

    let price = if let Some(price) = details.price {
        price
    } else {
        existing_course.price.unwrap_or_default()
    };

    let updated_at = Utc::now().naive_utc();
    let db_res = sqlx::query_as!(
        Course,
        "UPDATE courses set name = $1, description = $2, format = $3, 
        structure = $4, duration = $5, price = $6, language = $7, 
        level = $8, updated_at = $9 where id = $10 returning id, tutor_id, 
        name, description, duration, level, format, 
        language, structure, price, created_at, updated_at",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        updated_at,
        id,
    )
    .fetch_one(pool)
    .await?;

    Ok(db_res)
}

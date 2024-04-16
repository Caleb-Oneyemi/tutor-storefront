use sqlx::PgPool;

use crate::{
    errors::CustomError,
    models::tutors::{CreateTutor, Tutor, UpdateTutor},
};

pub async fn get_all(pool: &PgPool) -> Result<Vec<Tutor>, CustomError> {
    let db_res = sqlx::query_as!(Tutor, "SELECT * FROM tutors")
        .fetch_all(pool)
        .await?;

    Ok(db_res)
}

pub async fn create(pool: &PgPool, data: CreateTutor) -> Result<Tutor, CustomError> {
    let db_res = sqlx::query_as!(
        Tutor,
        "INSERT INTO tutors (name, bio) values ($1, $2) returning id, name, bio, photo_url, created_at",
        data.name,
        data.bio,
    )
    .fetch_one(pool)
    .await?;

    Ok(db_res)
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Tutor, CustomError> {
    let db_res = sqlx::query_as!(Tutor, "SELECT * FROM tutors where id = $1", id)
        .fetch_one(pool)
        .await?;

    Ok(db_res)
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    existing_tutor: Tutor,
    details: UpdateTutor,
) -> Result<Tutor, CustomError> {
    let name: String = if let Some(name) = details.name {
        name
    } else {
        existing_tutor.name
    };

    let bio: String = if let Some(bio) = details.bio {
        bio
    } else {
        existing_tutor.bio
    };

    let photo_url: String = if let Some(photo_url) = details.photo_url {
        photo_url
    } else {
        existing_tutor.photo_url.unwrap_or_default()
    };

    let db_res = sqlx::query_as!(Tutor, "UPDATE tutors set name = $1, bio = $2, photo_url = $3 where id = $4 returning id, name, bio, photo_url, created_at", name, bio, photo_url, id)
        .fetch_one(pool)
        .await?;

    Ok(db_res)
}

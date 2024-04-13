use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_all(pool: &PgPool) -> Vec<Course> {
    let courses = sqlx::query!("SELECT * from tutor_storefront")
        .fetch_all(pool)
        .await
        .unwrap();

    courses
        .iter()
        .map(|c| Course {
            id: Some(c.id),
            tutor_id: c.tutor_id,
            name: c.name.clone(),
            created_at: Some(chrono::NaiveDateTime::from(c.created_at.unwrap())),
        })
        .collect()
}

pub async fn get_by_tutor(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    let courses = sqlx::query!(
        "SELECT * from tutor_storefront where tutor_id = ($1)",
        tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    courses
        .iter()
        .map(|c| Course {
            id: Some(c.id),
            tutor_id: c.tutor_id,
            name: c.name.clone(),
            created_at: c.created_at,
        })
        .collect()
}

pub async fn create(pool: &PgPool, new_course: Course) -> Course {
    let course = sqlx::query!(
        "INSERT INTO tutor_storefront (tutor_id, name) values ($1, $2) returning id, tutor_id, name, created_at",
        new_course.tutor_id,
        new_course.name
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        id: Some(course.id),
        tutor_id: course.tutor_id,
        name: course.name,
        created_at: course.created_at,
    }
}

pub async fn _count_by_tutor(pool: &PgPool, tutor_id: i32) -> std::option::Option<i64> {
    let res = sqlx::query!(
        "SELECT COUNT(*) from tutor_storefront where tutor_id = ($1)",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    res.count
}

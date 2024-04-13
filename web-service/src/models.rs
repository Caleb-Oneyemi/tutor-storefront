use actix_web::web;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i32>,
    pub tutor_id: i32,
    pub name: String,
    pub created_at: Option<SystemTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(input: web::Json<Course>) -> Self {
        Course {
            id: input.id,
            tutor_id: input.tutor_id,
            name: input.name.clone(),
            created_at: input.created_at,
        }
    }
}

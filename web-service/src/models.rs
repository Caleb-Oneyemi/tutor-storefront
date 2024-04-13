use actix_web::web;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Course {
    id: Option<i32>,
    tutor_id: i32,
    name: String,
    created_at: Option<SystemTime>,
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

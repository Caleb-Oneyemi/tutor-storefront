use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx;
use std::str;

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub tutor_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
    pub duration: Option<i32>,
    pub price: Option<f32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
    pub duration: Option<i32>,
    pub price: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<i32>,
    pub price: Option<f32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(input: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            tutor_id: input.tutor_id,
            name: input.name.clone(),
            description: input.description.clone(),
            format: input.format.clone(),
            structure: input.structure.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            duration: input.duration,
            price: input.price,
        }
    }
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(input: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: input.name.clone(),
            description: input.description.clone(),
            format: input.format.clone(),
            structure: input.structure.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            duration: input.duration,
            price: input.price,
        }
    }
}

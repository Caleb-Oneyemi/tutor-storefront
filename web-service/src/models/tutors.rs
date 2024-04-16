use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct Tutor {
    pub id: i32,
    pub name: String,
    pub bio: String,
    pub photo_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateTutor {
    pub name: String,
    pub bio: String,
}

impl From<web::Json<CreateTutor>> for CreateTutor {
    fn from(val: web::Json<CreateTutor>) -> Self {
        CreateTutor {
            name: val.name.clone(),
            bio: val.bio.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateTutor {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub photo_url: Option<String>,
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(val: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            name: val.name.clone(),
            bio: val.bio.clone(),
            photo_url: val.photo_url.clone(),
        }
    }
}

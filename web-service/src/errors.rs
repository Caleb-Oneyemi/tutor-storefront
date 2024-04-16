use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum CustomError {
    ActixError(String),
    DbError(String),
    NotFoundError(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct CustomErrorResponse {
    message: String,
}

impl CustomError {
    fn get_response(&self) -> String {
        let default = "internal server error".into();

        match self {
            CustomError::ActixError(msg) => {
                println!("Server Error: {:?}", msg);
                default
            }
            CustomError::DbError(msg) => {
                println!("DB Error: {:?}", msg);
                default
            }
            CustomError::NotFoundError(msg) => {
                println!("Not Found Error: {:?}", msg);
                msg.into()
            }
            CustomError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::ActixError(_) | CustomError::DbError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            CustomError::NotFoundError(_) => StatusCode::NOT_FOUND,
            CustomError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(CustomErrorResponse {
            message: self.get_response(),
        })
    }
}

impl From<actix_web::error::Error> for CustomError {
    fn from(err: actix_web::error::Error) -> Self {
        CustomError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for CustomError {
    fn from(err: SQLxError) -> Self {
        CustomError::DbError(err.to_string())
    }
}

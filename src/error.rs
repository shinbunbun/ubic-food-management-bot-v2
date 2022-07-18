use std::env::VarError;

use log::error;

#[derive(Debug)]
pub enum AppError {
    Env(VarError),
    BadRequest(String),
    SerdeJson(serde_json::Error),
    LineBotSdkError(line_bot_sdk::Error),
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Env(_) | AppError::SerdeJson(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            AppError::LineBotSdkError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        error!("{}", self.to_string());
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Env(errors) => write!(f, "Env var Error: {}", errors),
            AppError::BadRequest(errors) => write!(f, "Bad Request: {}", errors),
            AppError::SerdeJson(errors) => write!(f, "serde json error: {}", errors),
            AppError::LineBotSdkError(errors) => write!(f, "line bot sdk error: {}", errors),
        }
    }
}

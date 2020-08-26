use actix_web::{
    dev::HttpResponseBuilder,
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InternalServerError(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    DbError(#[from] mongodb::error::Error),

    #[error(transparent)]
    SSLError(#[from] openssl::error::ErrorStack),

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    NotFound(String),

    #[error("You are unauthorized to access this data")]
    Forbidden,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IOError(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::DbError(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::SSLError(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Forbidden => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponseBuilder::new(status_code)
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(ErrorMessage {
                message: self.to_string(),
                status: status_code.to_string(),
            })
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
    status: String,
}

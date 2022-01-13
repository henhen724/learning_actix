use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppErrorType {
    ResourceNotFound,
    DBError,
}

pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    fn message(&self) -> String {
        match &*self {
            AppError{
                message: Some(message: &String),
                cause: _,
                error_type: _
            } => message.clone(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::ResourceNotFound } => "The resource you requested could not be found.".to_string(),
            _=> "An unknown error has occurred.".to_string()
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmr::Error> {
        write!(f, "{:?}", self);
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DBError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message,
        })
    }
}

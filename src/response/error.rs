use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use thiserror::Error;

use crate::response::ApiResponse;

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("Internal server error: {0}")]
  InternalServerError(String),
  #[error("Bad request: {0}")]
  BadRequest(String),
  #[error("Unauthorized: {0}")]
  Unauthorized(String),
  #[error("Forbidden: {0}")]
  Forbidden(String),
  #[error("Not found: {0}")]
  NotFound(String),
}

// Centralized API error definitions and conversion into the unified JSON response format.
impl IntoResponse for ApiError {
  fn into_response(self) -> Response {
    let (code, message) = match self {
      ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
      ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
      ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
      ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
      ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
    };
    (code, Json(ApiResponse::<()>::message(code, message))).into_response()
  }
}

impl From<sea_orm::DbErr> for ApiError {
  fn from(err: sea_orm::DbErr) -> Self {
      ApiError::InternalServerError(err.to_string())
  }
}

// Convert anyhow errors to ApiError
impl From<anyhow::Error> for ApiError {
  fn from(err: anyhow::Error) -> Self {
      ApiError::InternalServerError(err.to_string())
  }
}
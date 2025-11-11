use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where 
  T: Serialize,           // T must implement Serialize trait
{
  pub code: u16,
  pub message: String,
  pub data: Option<T>,
}

// Decentralize response construction logic into a single, centralized struct.
impl<T> ApiResponse<T>
where 
  T: Serialize 
{
  pub fn success(data: T) -> Self {
    Self {
      code: 200,
      message: "Success".into(),
      data: Some(data),
    }
  }

  pub fn message(status: StatusCode, msg: impl Into<String>) -> Self {
    Self {
        code: status.as_u16(),
        message: msg.into(),
        data: None,
    }
  }
} 

impl<T> IntoResponse for ApiResponse<T>
where 
  T: Serialize,
{
  fn into_response(self) -> Response {
    let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK);
    (status, Json(self)).into_response()
  }
}
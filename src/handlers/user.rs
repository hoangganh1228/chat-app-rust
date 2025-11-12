// chat-app-be/src/handlers/user.rs
use axum::{
  extract::State,
  http::HeaderMap,
};

use crate::{
  database::SharedState,
  dtos::room::UserInfo,
  response::{ApiError, ApiResponse},
  services::user,
};

fn extract_token(headers: &HeaderMap) -> Result<&str, ApiError> {
  let auth_header = headers
      .get("authorization")
      .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".into()))?
      .to_str()
      .map_err(|_| ApiError::Unauthorized("Invalid authorization header".into()))?;

  if !auth_header.starts_with("Bearer ") {
      return Err(ApiError::Unauthorized("Invalid authorization format".into()));
  }

  Ok(&auth_header[7..])
}

pub async fn list_all_users(
  State(state): State<SharedState>,
  headers: HeaderMap,
) -> Result<ApiResponse<Vec<UserInfo>>, ApiError> {
  // Xác thực token (bất kỳ user nào đăng nhập đều có thể xem danh sách users)
  let token = extract_token(&headers)?;
  let _user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  let users = user::list_all_users(state.as_ref()).await?;
  Ok(ApiResponse::success(users))
}
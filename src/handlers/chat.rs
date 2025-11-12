use axum::{
  extract::{Path, Query, State},
  http::{HeaderMap, StatusCode},
  Json,
};
use uuid::Uuid;

use crate::{
  database::SharedState,
  dtos::chat::{ListMessagesQuery, MessageResponse, SendMessageRequest},
  response::{ApiError, ApiResponse},
  services::chat,
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

  Ok(&auth_header[7..]) // Skip "Bearer "
}

pub async fn list_messages(
  State(state): State<SharedState>,
  Path(room_id): Path<Uuid>,
  headers: HeaderMap,
  Query(params): Query<ListMessagesQuery>,
) -> Result<ApiResponse<Vec<MessageResponse>>, ApiError> {
  let token = extract_token(&headers)?;
  let user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  let messages = chat::list_messages(state.as_ref(), room_id, user_id, params).await?;
  Ok(ApiResponse::success(messages))
}

pub async fn send_message(
  State(state): State<SharedState>,
  Path(room_id): Path<Uuid>,
  headers: HeaderMap,
  Json(payload): Json<SendMessageRequest>,
) -> Result<ApiResponse<MessageResponse>, ApiError> {
  let token = extract_token(&headers)?;
  let user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  let message =
      chat::send_message(state.as_ref(), room_id, user_id, payload.content).await?;

  let _ = state.chat_tx.send(message.clone());

  Ok(ApiResponse::success(message))
}

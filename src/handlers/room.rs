use axum::{
  extract::{Path, State},
  http::HeaderMap,
  Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
  database::SharedState,
  dtos::room::{AddMemberRequest, CreateRoomRequest, RoomDetailResponse, RoomResponse},
  response::{ApiError, ApiResponse},
  services::room,
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

pub async fn create_room(
  State(state): State<SharedState>,
  headers: HeaderMap,
  Json(payload): Json<CreateRoomRequest>,
) -> Result<ApiResponse<RoomResponse>, ApiError> {
  payload.validate()
      .map_err(|e| ApiError::BadRequest(format!("Validation error: {}", e)))?;

  let token = extract_token(&headers)?;
  let user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  let room = room::create_room(state.as_ref(), user_id, payload).await?;
  Ok(ApiResponse::success(room))
}

pub async fn get_room(
  State(state): State<SharedState>,
  Path(room_id): Path<Uuid>,
  headers: HeaderMap,
) -> Result<ApiResponse<RoomResponse>, ApiError> {
  let token = extract_token(&headers)?;
  let _user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  let room = room::get_room(state.as_ref(), room_id).await?;
  Ok(ApiResponse::success(room))
}

pub async fn get_room_detail(
  State(state): State<SharedState>,
  Path(room_id): Path<Uuid>,
  headers: HeaderMap,
) -> Result<ApiResponse<RoomDetailResponse>, ApiError> {
  let token = extract_token(&headers)?;
  let user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  let room = room::get_room_detail(state.as_ref(), room_id, user_id).await?;
  Ok(ApiResponse::success(room))
}

pub async fn list_rooms(
  State(state): State<SharedState>,
  headers: HeaderMap,
) -> Result<ApiResponse<Vec<RoomResponse>>, ApiError> {
  let token = extract_token(&headers)?;
  let user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  let rooms = room::list_rooms(state.as_ref(), user_id).await?;
  Ok(ApiResponse::success(rooms))
}

pub async fn delete_room(
  State(state): State<SharedState>,
  Path(room_id): Path<Uuid>,
  headers: HeaderMap,
) -> Result<ApiResponse<()>, ApiError> {
  let token = extract_token(&headers)?;
  let user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  room::delete_room(state.as_ref(), room_id, user_id).await?;
  Ok(ApiResponse::success(()))
}

pub async fn add_member(
  State(state): State<SharedState>,
  Path(room_id): Path<Uuid>,
  headers: HeaderMap,
  Json(payload): Json<AddMemberRequest>,
) -> Result<ApiResponse<()>, ApiError> {
  let token = extract_token(&headers)?;
  let requester_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;
  println!("requester_id: {:?}", requester_id);
  room::add_member(state.as_ref(), room_id, requester_id, payload).await?;
  Ok(ApiResponse::success(()))
}

pub async fn remove_member(
  State(state): State<SharedState>,
  Path((room_id, user_id)): Path<(Uuid, Uuid)>,
  headers: HeaderMap,
) -> Result<ApiResponse<()>, ApiError> {
  let token = extract_token(&headers)?;
  let requester_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  room::remove_member(state.as_ref(), room_id, requester_id, user_id).await?;
  Ok(ApiResponse::success(()))
}


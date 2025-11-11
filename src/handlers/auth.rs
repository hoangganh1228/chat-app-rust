use axum::{extract::State, Json};
use validator::Validate;
use crate::{
    database::SharedState,
    dtos::auth::{LoginRequest, RegisterRequest, AuthResponse, LoginResponse, UserResponse},
    response::{ApiError, ApiResponse},
    services::auth,
};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    payload.validate()
        .map_err(|e| ApiError::BadRequest(format!("Validation error: {}", e)))?;
    auth::register(&state, payload).await
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, ApiError> {
    payload.validate()
        .map_err(|e| ApiError::BadRequest(format!("Validation error: {}", e)))?;
    auth::login(&state, payload).await
}
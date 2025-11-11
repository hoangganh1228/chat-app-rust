use axum::{extract::State, Json};

use crate::{
    database::SharedState,
    dtos::auth::{LoginRequest, RegisterRequest, AuthResponse},
    response::{ApiError, ApiResponse},
    services::auth,
};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ApiResponse<AuthResponse>, ApiError> {
    auth::register(&state, payload).await
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<ApiResponse<AuthResponse>, ApiError> {
    auth::login(&state, payload).await
}
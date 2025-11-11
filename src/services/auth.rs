use anyhow::Context;    // Collect errors from multiple sources and return a single error
use uuid::Uuid;
use chrono::Utc;

use crate::{
  database::AppState,
  dtos::auth::{AuthResponse, UserResponse, RegisterRequest, LoginRequest},
  entities::user::User,
  repositories::user as user_repo,
  response::{ApiError, ApiResponse},
  security::{hash_password, verify_password},
};

pub async fn register(state: &AppState, req: RegisterRequest) -> Result<ApiResponse<AuthResponse>, ApiError> {
  if let Some(_) = user_repo::find_by_email(&state.db, &req.email).await? {
    return Err(ApiError::BadRequest("Email already in use".to_string()));
  }

  let hashed = hash_password(&req.password).context("Failed to hash password")?;
  let user = User {
    id: Uuid::new_v4(),
    username: req.username,
    email: req.email,
    password: hashed,
    created_at: Utc::now(),
  };

  user_repo::insert(&state.db, &user).await?;
  let token = state.jwt.generate(user.id).context("generate jwt")?;

  let payload = AuthResponse {
      token,
      user: UserResponse {
          id: user.id,
          username: user.username,
          email: user.email,
          created_at: user.created_at,
      },
  };
  Ok(ApiResponse::success(payload))
}

pub async fn login(state: &AppState, req: LoginRequest)
    -> Result<ApiResponse<AuthResponse>, ApiError>
{
  let user = user_repo::find_by_email(&state.db, &req.email)
    .await?
    .ok_or_else(|| ApiError::Unauthorized("Invalid credentials".into()))?;

  if !verify_password(&req.password, &user.password).context("verify password")? {
    return Err(ApiError::Unauthorized("Invalid credentials".into()));
  }

  let token = state.jwt.generate(user.id).context("generate jwt")?;
  let payload = AuthResponse {
      token,
      user: UserResponse {
          id: user.id,
          username: user.username,
          email: user.email,
          created_at: user.created_at,
      },
  };

  Ok(ApiResponse::success(payload))
}
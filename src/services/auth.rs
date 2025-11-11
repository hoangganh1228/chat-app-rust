use anyhow::Context;
use uuid::Uuid;
use chrono::Utc;

use crate::{
  database::AppState,
  dtos::auth::{AuthResponse, UserResponse, RegisterRequest, LoginRequest, LoginResponse},
  entities::user::Model as UserModel,
  repositories::user as user_repo,
  response::{ApiError, ApiResponse},
  security::{hash_password, verify_password},
};

pub async fn register(state: &AppState, req: RegisterRequest) -> Result<ApiResponse<UserResponse>, ApiError> {
  // Check if user already exists
  if let Some(_) = user_repo::find_by_email(&state.db, &req.email).await? {
    return Err(ApiError::BadRequest("Email already in use".to_string()));
  }

  let hashed = hash_password(&req.password).context("Failed to hash password")?;

  let user = UserModel {
    id: Uuid::new_v4().to_string(),
    username: req.username,
    email: req.email.clone(),
    password: hashed,
    created_at: Utc::now(),
  };
  
  let user = user_repo::insert(&state.db, user).await?;
  let user_id = Uuid::parse_str(&user.id)
      .context("Invalid user ID format")?;
  
  let token = state.jwt.generate(user_id).context("Failed to generate JWT")?;
  
  let payload = UserResponse {
      id: user_id,  
      username: user.username,
      email: user.email,
      created_at: user.created_at,
  };

  Ok(ApiResponse::success(payload))
}

pub async fn login(state: &AppState, req: LoginRequest) -> Result<ApiResponse<LoginResponse>, ApiError> {
    let user = user_repo::find_by_email(&state.db, &req.email)
        .await?
        .ok_or_else(|| ApiError::Unauthorized("Invalid credentials".into()))?;

    if !verify_password(&req.password, &user.password).context("Failed to verify password")? {
        return Err(ApiError::Unauthorized("Invalid credentials".into()));
    }

    let user_id = Uuid::parse_str(&user.id)
        .context("Invalid user ID format")?;
    
    let token = state.jwt.generate(user_id).context("Failed to generate JWT")?;
    
    let payload = LoginResponse {
        token,
    };

    Ok(ApiResponse::success(payload))
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 3, max = 100, message = "Password must be between 3 and 100 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 3, max = 100, message = "Password must be between 3 and 100 characters"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
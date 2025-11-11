use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub password: String,
  pub created_at: DateTime<Utc>,
}
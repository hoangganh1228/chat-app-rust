use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RoomMember {
  pub room_id: Uuid,
  pub user_id: Uuid,
  pub joined_at: DateTime<Utc>,
}
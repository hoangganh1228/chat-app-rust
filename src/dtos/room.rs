use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize)]
pub struct RoomResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateRoomRequest {
    #[validate(length(min = 1, max = 100, message = "Room name must be between 1 and 100 characters"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomMemberResponse {
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomDetailResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub members: Vec<UserInfo>,
}
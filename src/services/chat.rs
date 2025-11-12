use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use crate::{
    database::AppState,
    dtos::chat::{ListMessagesQuery, MessageDto},
    entities::{
        message::{ActiveModel as MessageActiveModel, Column as MessageColumn, Entity as MessageEntity, Model as MessageModel},
        room_member::Entity as RoomMemberEntity,
    },
    response::ApiError,
};

// Fetch messages from a room, only if the user is a member.
// Limit the number of messages and order by creation time.
pub async fn list_messages(
  state: &AppState,
  room_id: Uuid,
  user_id: Uuid,
  params: ListMessagesQuery,
) -> Result<Vec<MessageDto>, ApiError> {
  ensure_membership(state, room_id, user_id).await?;

  let limit = params.limit.unwrap_or(50).min(200);

  let models = MessageEntity::find()
      .filter(MessageColumn::RoomId.eq(room_id.to_string()))
      .order_by_asc(MessageColumn::CreatedAt)
      .limit(limit)
      .all(&state.db)
      .await?;

  models.into_iter().map(to_dto).collect()
}

// Insert a new message into the DB if the user is a member of the room.

pub async fn send_message(
  state: &AppState,
  room_id: Uuid,
  sender_id: Uuid,
  content: String,
) -> Result<MessageDto, ApiError> {
  ensure_membership(state, room_id, sender_id).await?;

  let trimmed = content.trim();
  if trimmed.is_empty() {
      return Err(ApiError::BadRequest("Message content cannot be empty".into()));
  }
  if trimmed.len() > 1024 {
      return Err(ApiError::BadRequest("Message content is too long (max 1024 chars)".into()));
  }

  let created_at = Utc::now();
  let model = MessageActiveModel {
      id: Set(Uuid::new_v4().to_string()),
      room_id: Set(room_id.to_string()),
      sender_id: Set(sender_id.to_string()),
      content: Set(trimmed.to_owned()),
      created_at: Set(created_at),
  }
  .insert(&state.db)
  .await?;

  to_dto(model)
}

// Check if the user is a member of the room.
pub async fn ensure_membership(
  state: &AppState,
  room_id: Uuid,
  user_id: Uuid,
) -> Result<(), ApiError> {
  let exists = RoomMemberEntity::find_by_id((room_id.to_string(), user_id.to_string()))
      .one(&state.db)
      .await?;
  if exists.is_none() {
      return Err(ApiError::Forbidden("You are not a member of this room".into()));
  }
  Ok(())
}

// Convert a DB model into API DTO format, parsing string IDs to UUIDs.
fn to_dto(model: MessageModel) -> Result<MessageDto, ApiError> {
  Ok(MessageDto {
      id: Uuid::parse_str(&model.id)
          .map_err(|_| ApiError::InternalServerError("Invalid message id".into()))?,
      room_id: Uuid::parse_str(&model.room_id)
          .map_err(|_| ApiError::InternalServerError("Invalid room id".into()))?,
      sender_id: Uuid::parse_str(&model.sender_id)
          .map_err(|_| ApiError::InternalServerError("Invalid sender id".into()))?,
      content: model.content,
      created_at: model.created_at,
  })
}




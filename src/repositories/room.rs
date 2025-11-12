use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use uuid::Uuid;

use crate::{
    database::DbPool,
    entities::room::{ActiveModel, Entity as RoomEntity, Model as RoomModel},
};

pub async fn find_by_id(db: &DbPool, room_id: &str) -> Result<RoomModel, sea_orm::DbErr> {
  RoomEntity::find_by_id(room_id)
    .one(db)
    .await?
    .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("Room not found: {}", room_id)))
}

pub async fn insert(db: &DbPool, room: RoomModel) -> Result<RoomModel, sea_orm::DbErr> {
  let room_id = room.id.clone();
  let active_model = ActiveModel {
    id: Set(room.id.clone()),
    name: Set(room.name.clone()),
    created_at: Set(room.created_at),
  };

  let _ = active_model.insert(db).await;
  RoomEntity::find_by_id(&room_id)
    .one(db)
    .await?
    .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("Failed to find inserted room with id: {}", room_id)))
}

pub async fn delete(db: &DbPool, room_id: &str) -> Result<(), sea_orm::DbErr> {
  let room = RoomEntity::find_by_id(room_id)
      .one(db)
      .await?
      .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("Room not found: {}", room_id)))?;

  let active_model: ActiveModel = room.into();
  active_model.delete(db).await?;
  Ok(())
}

pub async fn list_all(db: &DbPool) -> Result<Vec<RoomModel>, sea_orm::DbErr> {
  RoomEntity::find().all(db).await
}
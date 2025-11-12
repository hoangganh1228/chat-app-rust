use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use chrono::Utc;

use crate::{
  database::DbPool,
  entities::room_member::{ActiveModel, Entity as RoomMemberEntity, Model as RoomMemberModel, Column},
};

pub async fn find_by_room_and_user(
  db: &DbPool,
  room_id: &str,
  user_id: &str,
) -> Result<RoomMemberModel, sea_orm::DbErr> {
  RoomMemberEntity::find_by_id((room_id.to_string(), user_id.to_string()))
    .one(db)
    .await?
    .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("Room member not found:")))
}

pub async fn insert(
  db: &DbPool,
  room_id: String,
  user_id: String,
) -> Result<RoomMemberModel, sea_orm::DbErr> {
  let active_model = ActiveModel {
    room_id: Set(room_id.clone()),
    user_id: Set(user_id.clone()),
    joined_at: Set(Utc::now()),
  };
  
  let result = active_model.insert(db).await?;
  Ok(result)
}

pub async fn delete(
  db: &DbPool,
  room_id: &str,
  user_id: &str,
) -> Result<(), sea_orm::DbErr> {
  let member = RoomMemberEntity::find_by_id((room_id.to_string(), user_id.to_string()))
      .one(db)
      .await?
      .ok_or_else(|| sea_orm::DbErr::RecordNotFound("Room member not found".to_string()))?;

  let active_model: ActiveModel = member.into();
  active_model.delete(db).await?;
  Ok(())
}

pub async fn list_by_room(
  db: &DbPool,
  room_id: &str,
) -> Result<Vec<RoomMemberModel>, sea_orm::DbErr> {
  RoomMemberEntity::find()
      .filter(Column::RoomId.eq(room_id))
      .all(db)
      .await
}

pub async fn list_by_user(
  db: &DbPool,
  user_id: &str,
) -> Result<Vec<RoomMemberModel>, sea_orm::DbErr> {
  RoomMemberEntity::find()
      .filter(Column::UserId.eq(user_id))
      .all(db)
      .await
}

use uuid::Uuid;
use sea_orm::EntityTrait;

use crate::{
    database::AppState,
    dtos::room::{AddMemberRequest, CreateRoomRequest, RoomDetailResponse, RoomResponse, UserInfo},
    entities::user::Entity as UserEntity,
    repositories::{
        room as room_repo, room_member as member_repo, user as user_repo,
    },
    response::ApiError,
};

pub async fn create_room(
  state: &AppState,
  creator_id: Uuid,
  req: CreateRoomRequest,
) -> Result<RoomResponse, ApiError> {
  let room = crate::entities::room::Model {
    id: Uuid::new_v4().to_string(),
    name: req.name,
    created_at: chrono::Utc::now(),
};
  let room = room_repo::insert(&state.db, room).await?;
  member_repo::insert(&state.db, room.id.clone(), creator_id.to_string()).await?;

  Ok(RoomResponse {
      id: Uuid::parse_str(&room.id)
          .map_err(|_| ApiError::InternalServerError("Invalid room id".into()))?,
      name: room.name,
      created_at: room.created_at,
  })
}

pub async fn get_room(state: &AppState, room_id: Uuid) -> Result<RoomResponse, ApiError> {
  let room = room_repo::find_by_id(&state.db, &room_id.to_string()).await?;

  Ok(RoomResponse {
      id: Uuid::parse_str(&room.id)
          .map_err(|_| ApiError::InternalServerError("Invalid room id".into()))?,
      name: room.name,
      created_at: room.created_at,
  })
}

pub async fn get_room_detail(
  state: &AppState,
  room_id: Uuid,
  user_id: Uuid,
) -> Result<RoomDetailResponse, ApiError> {
  let member = member_repo::find_by_room_and_user(&state.db, &room_id.to_string(), &user_id.to_string())
      .await?;

  let room = room_repo::find_by_id(&state.db, &room_id.to_string()).await?;
  let members = member_repo::list_by_room(&state.db, &room_id.to_string()).await?;

  let mut user_infos = Vec::new();
    for member in members {
        let user = UserEntity::find_by_id(&member.user_id)
            .one(&state.db)
            .await?
            .ok_or_else(|| ApiError::NotFound("User not found".into()))?;

        user_infos.push(UserInfo {
            id: Uuid::parse_str(&user.id)
                .map_err(|_| ApiError::InternalServerError("Invalid user id".into()))?,
            username: user.username,
            email: user.email,
        });
    }

    Ok(RoomDetailResponse {
        id: Uuid::parse_str(&room.id)
            .map_err(|_| ApiError::InternalServerError("Invalid room id".into()))?,
        name: room.name,
        created_at: room.created_at,
        members: user_infos,
    })

}


pub async fn list_rooms(state: &AppState, user_id: Uuid) -> Result<Vec<RoomResponse>, ApiError> {
  let user_members = member_repo::list_by_user(&state.db, &user_id.to_string()).await?;

  let mut rooms = Vec::new();
  for member in user_members {
      let room = room_repo::find_by_id(&state.db, &member.room_id).await?;
      rooms.push(RoomResponse {
          id: Uuid::parse_str(&room.id)
              .map_err(|_| ApiError::InternalServerError("Invalid room id".into()))?,
          name: room.name,
          created_at: room.created_at,
      });
  }

  Ok(rooms)
}

pub async fn delete_room(
  state: &AppState,
  room_id: Uuid,
  user_id: Uuid,
) -> Result<(), ApiError> {
  // Kiểm tra user có trong room không (có thể thêm logic: chỉ creator mới được xóa)
  let member = member_repo::find_by_room_and_user(&state.db, &room_id.to_string(), &user_id.to_string())
      .await?;

  room_repo::delete(&state.db, &room_id.to_string()).await?;
  Ok(())
}

pub async fn add_member(
  state: &AppState,
  room_id: Uuid,
  requester_id: Uuid,
  req: AddMemberRequest,
) -> Result<(), ApiError> {
  // Kiểm tra requester có trong room không
  let requester_member = member_repo::find_by_room_and_user(
      &state.db,
      &room_id.to_string(),
      &requester_id.to_string(),
  )
  .await?;

  // if requester_member.is_none() {
  //     return Err(ApiError::Forbidden("You are not a member of this room".into()));
  // }

  // let _user = user_repo::find_by_email(&state.db, "").await?; 
  let user = UserEntity::find_by_id(&req.user_id.to_string())
      .one(&state.db)
      .await?
      .ok_or_else(|| ApiError::NotFound("User not found".into()))?;
  
  let existing = member_repo::find_by_room_and_user(
      &state.db,
      &room_id.to_string(),
      &req.user_id.to_string(),
  )
  .await;

  // if existing.is_some() {
  //     return Err(ApiError::BadRequest("User is already a member of this room".into()));
  // }

  member_repo::insert(&state.db, room_id.to_string(), req.user_id.to_string()).await?;
  Ok(())
}

pub async fn remove_member(
  state: &AppState,
  room_id: Uuid,
  requester_id: Uuid,
  target_user_id: Uuid,
) -> Result<(), ApiError> {
  // Kiểm tra requester có trong room không
  let requester_member = member_repo::find_by_room_and_user(
      &state.db,
      &room_id.to_string(),
      &requester_id.to_string(),
  )
  .await?;

  // Không cho phép tự xóa chính mình (hoặc có thể cho phép)
  if requester_id == target_user_id {
      return Err(ApiError::BadRequest("You cannot remove yourself".into()));
  }

  member_repo::delete(&state.db, &room_id.to_string(), &target_user_id.to_string()).await?;
  Ok(())
}
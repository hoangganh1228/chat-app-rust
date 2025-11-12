use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::{
    database::AppState,
    dtos::room::UserInfo,
    entities::user::Entity as UserEntity,
    response::ApiError,
};

pub async fn list_all_users(state: &AppState) -> Result<Vec<UserInfo>, ApiError> {
    let users = UserEntity::find()
        .all(&state.db)
        .await?;

    let mut user_infos = Vec::new();
    for user in users {
        user_infos.push(UserInfo {
            id: Uuid::parse_str(&user.id)
                .map_err(|_| ApiError::InternalServerError("Invalid user id".into()))?,
            username: user.username,
            email: user.email,
        });
    }

    Ok(user_infos)
}
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, DbErr, ActiveModelTrait, Set, NotSet};
use uuid::Uuid;
use chrono::Utc;

use crate::{
    database::DbPool,
    entities::user::{Entity as UserEntity, Model as UserModel, ActiveModel, Column},
};

pub async fn find_by_email(db: &DbPool, email: &str) -> Result<Option<UserModel>, DbErr> {
    UserEntity::find()
    .filter(Column::Email.eq(email))
    .one(db)
    .await
}

pub async fn insert(db: &DbPool, user: UserModel) -> Result<UserModel, DbErr> {
    let user_id = user.id.clone();
    let active_model = ActiveModel {
        id: Set(user.id.clone()),
        username: Set(user.username.clone()),
        email: Set(user.email.clone()),
        password: Set(user.password.clone()),
        created_at: NotSet,
    };
    
    let _ = active_model.insert(db).await;
    
    UserEntity::find_by_id(&user_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("Failed to find inserted user with id: {}", user_id)))
}

pub async fn list_all(db: &DbPool) -> Result<Vec<UserModel>, DbErr> {
    UserEntity::find().all(db).await
}


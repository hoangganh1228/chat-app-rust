
use crate::{database::DbPool, entities::user::User};

pub async fn find_by_email(pool: &DbPool, email: &str) -> sqlx::Result<Option<User>> {
  sqlx::query_as::<_, User>(
      "SELECT id, username, email, password, created_at FROM users WHERE email = ?"
  )
  .bind(email)
  .fetch_optional(pool)
  .await
}

pub async fn insert(
  pool: &DbPool,
  user: &User,
) -> sqlx::Result<()> {
  sqlx::query(
      "INSERT INTO users (id, username, email, password, created_at) VALUES (?, ?, ?, ?, ?)"
  )
  .bind(user.id)
  .bind(&user.username)
  .bind(&user.email)
  .bind(&user.password)
  .bind(user.created_at)
  .execute(pool)
  .await
  .map(|_| ())
}
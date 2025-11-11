use std::sync::Arc;   // Arc = Atomic Reference Counted pointer, share data across threads/tasks safely
use std::time::Duration; 
use crate::security::JwtManager;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub type DbPool = Pool<MySql>;
pub type SharedState = Arc<AppState>;

#[derive(Debug, Clone)]
pub struct AppState {
  pub db: DbPool,
  pub jwt: JwtManager,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
  pub database_url: String,
  pub max_connections: u32,
}

impl DatabaseConfig {
  pub fn from_env() -> anyhow::Result<Self> {
    let database_url = if let Ok(url) = std::env::var("DATABASE_URL") {
        url
    } else {
        let host = std::env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = std::env::var("DB_PORT").unwrap_or_else(|_| "3306".into());
        let user = std::env::var("DB_USER").unwrap_or_else(|_| "root".into());
        let password = std::env::var("DB_PASSWORD").unwrap_or_default();
        let name = std::env::var("DB_NAME").unwrap_or_else(|_| "chat_app".into());

        if password.is_empty() {
            format!("mysql://{user}@{host}:{port}/{name}")
        } else {
            format!("mysql://{user}:{password}@{host}:{port}/{name}")
        }
    };

    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);

    Ok(Self {
        database_url,
        max_connections,
    })
  }

  pub async fn connect(&self) -> sqlx::Result<DbPool> {
      MySqlPoolOptions::new()
          .max_connections(self.max_connections)
          .connect(&self.database_url)
          .await
  }
}

pub async fn init_db_pool() -> anyhow::Result<DbPool> {
  let config = DatabaseConfig::from_env()?;
  let pool = config.connect().await?;
  Ok(pool)
}

pub async fn init_app_state(
    jwt_secret: String,
    jwt_expiration: Duration,
) -> anyhow::Result<SharedState> {
  let db = init_db_pool().await?;
  let jwt = JwtManager::new(jwt_secret, jwt_expiration);
  let state = Arc::new(AppState { db, jwt });
  Ok(state)
}
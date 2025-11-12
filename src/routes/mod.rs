pub mod auth;
pub mod chat;
use axum::Router;
use crate::database::SharedState;

pub fn build() -> Router<SharedState> {
  Router::new()
      .merge(auth::router())
      .merge(chat::router())
}
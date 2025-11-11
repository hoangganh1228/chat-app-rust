pub mod auth;
use axum::Router;
use crate::database::SharedState;

pub fn build() -> Router<SharedState> {
  Router::new()
      .merge(auth::router())
}
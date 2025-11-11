use axum::{routing::post, Router};

use crate::{database::SharedState, handlers};

pub fn router() -> Router<SharedState> {
  Router::new()
    .route("/auth/register", post(handlers::auth::register))
    .route("/auth/login", post(handlers::auth::login))
}
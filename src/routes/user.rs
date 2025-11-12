use axum::{
  routing::get,
  Router,
};

use crate::{database::SharedState, handlers};

pub fn router() -> Router<SharedState> {
  Router::new()
    .route("/users", get(handlers::user::list_all_users))
}
use axum::{
  routing::{get, post, delete},
  Router,
};

use crate::{database::SharedState, handlers};

pub fn router() -> Router<SharedState> {
  Router::new()
    .route("/rooms/:room_id/messages", get(handlers::chat::list_messages).post(handlers::chat::send_message))
    .route("/ws", get(handlers::ws::upgrade))
    .route("/rooms", post(handlers::room::create_room).get(handlers::room::list_rooms))
    .route("/rooms/:room_id", get(handlers::room::get_room).delete(handlers::room::delete_room))
    .route("/rooms/:room_id/detail", get(handlers::room::get_room_detail))
    .route("/rooms/:room_id/members", post(handlers::room::add_member))
    .route("/rooms/:room_id/members/:user_id", delete(handlers::room::remove_member))
}
use axum::{
  extract::{Query, State},
  response::IntoResponse,
  http::HeaderMap,
};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};   // use WebSocketUpgrade to upgrade a HTTP request to a WebSocket connection
use futures::{StreamExt, SinkExt};
use serde::Deserialize;
use tokio::select;      // run multiple futures concurrently and handle the results
use tokio_stream::wrappers::BroadcastStream; // Wrapper to convert a broadcast channel into a stream
use uuid::Uuid;

use crate::{
  database::SharedState,
  dtos::chat::{WsInboundMessage, WsOutboundMessage},
  response::ApiError,
  services::chat,
};

#[derive(Debug, Deserialize)]
pub struct WsConnectQuery {
    pub room_id: Uuid,
}

fn extract_token(headers: &HeaderMap) -> Result<&str, ApiError> {
  let auth_header = headers
      .get("authorization")
      .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".into()))?
      .to_str()
      .map_err(|_| ApiError::Unauthorized("Invalid authorization header".into()))?;

  if !auth_header.starts_with("Bearer ") {
      return Err(ApiError::Unauthorized("Invalid authorization format".into()));
  }

  Ok(&auth_header[7..]) // Skip "Bearer "
}

pub async fn upgrade (
  State(state): State<SharedState>,
  headers: HeaderMap,
  Query(params): Query<WsConnectQuery>,
  ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, ApiError> {
  let token = extract_token(&headers)?;
  let user_id = state
      .jwt
      .validate(token)
      .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

  chat::ensure_membership(state.as_ref(), params.room_id, user_id).await?;

  Ok(ws.on_upgrade(move |socket| handle_socket(state, user_id, params.room_id, socket)))
}

async fn handle_socket(
  state: SharedState,
  user_id: Uuid,
  room_id: Uuid,
  socket: WebSocket,
) {
  let (mut ws_sender, mut ws_receiver) = socket.split();
  let mut rx_stream = BroadcastStream::new(state.chat_tx.subscribe());
  let reader_state = state.clone();

  let mut read_task = tokio::spawn(async move {
    while let Some(Ok(msg)) = ws_receiver.next().await {
        match msg {
            Message::Text(text) => {
                if let Ok(payload) = serde_json::from_str::<WsInboundMessage>(&text) {
                    if payload.content.trim().is_empty() {
                        continue;
                    }
                    if let Ok(message) = chat::send_message(
                        reader_state.as_ref(),
                        room_id,
                        user_id,
                        payload.content,
                    )
                    .await
                    {
                        let _ = reader_state.chat_tx.send(message);
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
  });

  let mut write_task = tokio::spawn(async move {
    while let Some(Ok(event)) = rx_stream.next().await {
        if event.room_id != room_id {
            continue;
        }
        if ws_sender
            .send(Message::Text(match serde_json::to_string(&event) {
                Ok(json) => json,
                Err(_) => continue,
            }))
            .await
            .is_err()
        {
            break;
        }
      }
  });

  select! {
      _ = &mut read_task => {},
      _ = &mut write_task => {},
  }

  read_task.abort();
  write_task.abort();
}
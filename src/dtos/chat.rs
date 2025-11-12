use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

pub type WsInboundMessage = SendMessageRequest;

#[derive(Debug, Clone, Serialize)]
pub struct MessageDto {
    pub id: Uuid,
    pub room_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

pub type MessageResponse = MessageDto;
pub type WsOutboundMessage = MessageDto;

#[derive(Debug, Clone, Deserialize)]
pub struct ListMessagesQuery {
    pub limit: Option<u64>,
}
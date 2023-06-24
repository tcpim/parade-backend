use crate::api_interface::common_interface::Cursor;
use crate::models::chat_model::ChatClubMessage;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct SendClubMessage {
    pub message_id: String,
    pub sender: String,
    pub created_ts: u64,
    pub words: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UpdateClubMessage {
    pub message_id: String,
    pub words: String,
    pub updater: String, // user id
    pub updated_ts: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct DeleteClubMessage {
    pub message_id: String,
    pub deleter: String, // user id
    pub deleted_ts: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct ReactClubMessage {
    pub message_id: String,
    pub emoji: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetClubMessagesRequest {
    pub cursor: Cursor<u32>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetClubMessagesResponse {
    pub next_cursor: Cursor<u32>,
    pub messages: Vec<ChatClubMessage>,
}

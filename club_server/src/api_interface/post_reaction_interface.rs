use super::common_interface::ServerError;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct ReactEmojiRequest {
    pub user: String,             // user who reacted
    pub post_id: Option<String>,  // if set, the post being reacted. See Post.id
    pub reply_id: Option<String>, // if set, the reply being reacted. See Reply.id
    pub emoji: String,            // the emoji reacted
    pub created_ts: u64,          // the timestamp when this reaction is created
}

#[derive(Debug, CandidType, Deserialize)]
pub struct ReactEmojiResponse {
    pub error: Option<ServerError>,
}

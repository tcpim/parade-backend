use super::common::ServerError;
use crate::models::nft::NftToken;
use crate::models::post::PostReply;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct ReplyPostRequest {
    pub reply_id: String,    // the reply id, uuid from frontend
    pub user: String,        // user id who replied
    pub post_id: String,     // the post being replied. See Post.id
    pub words: String,       // the reply content
    pub created_ts: u64,     // the timestamp when this reply is created
    pub nfts: Vec<NftToken>, // user can reply with NFTs
}

#[derive(Debug, CandidType, Deserialize, PartialEq)]
pub struct ReplyPostResponse {
    pub reply: PostReply,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetPostRepliesRequest {
    pub post_id: String,
    pub offset: i32,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetPostRepliesResponse {
    pub post_replies: Vec<PostReply>,
    pub offset: i32,
    pub error: Option<ServerError>,
}

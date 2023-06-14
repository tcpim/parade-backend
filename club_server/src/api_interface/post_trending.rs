use crate::api_interface::common::{Cursor, ServerError};
use crate::models::post::Post;
use crate::models::trending_post::TrendingPostKey;
use crate::models::trending_post_collection::TrendingPostCollectionKey;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct GetTrendingPostRequest {
    pub cursor: Cursor<TrendingPostKey>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetTrendingPostResponse {
    pub posts: Vec<Post>,
    pub next_cursor: Cursor<TrendingPostKey>,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetTrendingCollectionPostRequest {
    pub canister_id: String,
    pub cursor: Cursor<TrendingPostCollectionKey>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetTrendingCollectionPostResponse {
    pub posts: Vec<Post>,
    pub next_cursor: Cursor<TrendingPostCollectionKey>,
    pub error: Option<ServerError>,
}

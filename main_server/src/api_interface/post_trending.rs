use crate::api_interface::common::{Cursor, ServerError};

use crate::models::trending_post::TrendingPostKey;
use crate::models::trending_post_collection::TrendingPostCollectionKey;
use candid::CandidType;
use serde::Deserialize;
use crate::api_interface::posts::PostType;

#[derive(Debug, CandidType, Deserialize)]
pub struct GetTrendingStreetPostRequest {
    pub cursor: Cursor<TrendingPostKey>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetTrendingStreetPostResponse {
    pub posts: Vec<PostType>,
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
    pub posts: Vec<PostType>,
    pub next_cursor: Cursor<TrendingPostCollectionKey>,
    pub error: Option<ServerError>,
}

use crate::api_interface::common::{Cursor, ServerError};

use crate::api_interface::posts::PostType;
use crate::models::post::Post;
use crate::models::trending_post::TrendingPostKey;
use crate::models::trending_post_collection::TrendingPostCollectionKey;
use candid::CandidType;
use serde::Deserialize;

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

#[derive(Debug, CandidType, Deserialize)]
pub struct UpdateClubPostStreetTrendingScoreRequest {
    pub new: TrendingPostKey,
    pub nft_canister_ids: Vec<String>,
}

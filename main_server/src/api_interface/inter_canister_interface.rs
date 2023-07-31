use crate::models::nft_model::NftToken;
use crate::models::post_user_model::UserPostCreatedTsKey;
use crate::models::trending_post_model::TrendingPostKey;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct AddClubPostToUserRequest {
    pub caller: String,                              // the original canister caller
    pub user_post_created_key: UserPostCreatedTsKey, // the user post created key
}

#[derive(Debug, CandidType, Deserialize)]
pub struct AddClubPostToStreetRequest {
    pub post_id: String,
    pub club_id: String,
    pub nfts: Vec<NftToken>,
    pub created_ts: u64,
    pub created_by: String,
    pub caller: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UpdateClubPostStreetTrendingScoreRequest {
    pub new: TrendingPostKey,
    pub nft_canister_ids: Vec<String>,
    pub caller: String,
}

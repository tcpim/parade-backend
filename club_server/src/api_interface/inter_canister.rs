use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct AddClubPostToStreetRequest {
    pub post_id: String,
    pub club_id: String,
    pub nfts: Vec<NftTokenExternal>,
    pub created_ts: u64,
    pub created_by: String,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq)]
pub struct NftTokenExternal {
    pub canister_id: String,
    pub token_index: u16,
    pub token_id: String,
    pub collection_name: String,
    pub original_image_url: String,
    pub original_thumbnail_url: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UserPostCreatedTsKeyExternal {
    pub user_id: String,
    pub created_ts: u64,
    pub post_id: String,
    pub club_id: Option<String>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct TrendingPostKeyExternal {
    pub post_id: String,
    pub trending_score: u32,
    pub created_ts: u64,
    pub updated_ts: u64,
    pub club_id: Option<String>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UpdateClubPostStreetTrendingScoreRequest {
    pub new: TrendingPostKeyExternal,
    pub nft_canister_ids: Vec<String>,
}

use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct AddClubPostToStreetRequest {
    pub post_id: String,
    pub club_id: String,
    pub nfts: Vec<NftToken>,
    pub created_ts: u64,
    pub created_by: String,
}

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq)]
pub struct NftToken {
    pub canister_id: String,
    pub token_index: u16,
    pub token_id: String,
    pub collection_name: String,
    pub original_image_url: String,
    pub original_thumbnail_url: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UserAddPostRequest {
    pub user_id: String, // the user principal
    pub post_id: String,
    pub club_id: Option<String>,
    pub created_ts: u64,
}

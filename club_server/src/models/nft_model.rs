use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq)]
pub struct NftToken {
    pub canister_id: String,
    pub token_index: u16,
    pub token_id: String,
    pub collection_name: String,
    pub image_url: String,
    pub image_onchain_url: String,
    pub image_height_width_ratio: String,
    pub image_type: String,
    pub image_thumbnail_url: String,
    pub club_id: String,
}

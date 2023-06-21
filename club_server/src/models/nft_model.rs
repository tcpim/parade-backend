use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq)]
pub struct NftToken {
    pub canister_id: String,
    pub token_index: u16,
    pub token_id: String,
    pub collection_name: String,
    pub original_image_url: String,
    pub original_thumbnail_url: String,
}

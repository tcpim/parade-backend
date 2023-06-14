use crate::api_interface::posts::*;
use crate::models::nft::NftToken;

// Helper methods for generating requests
pub fn generate_create_post_request(
    created_ts: u64,
    words: String,
    created_by: String,
    nfts: Vec<NftToken>,
) -> CreateStreetPostRequest {
    CreateStreetPostRequest {
        post_id: created_ts.to_string(),
        created_by,
        nfts,
        words,
        created_ts,
    }
}

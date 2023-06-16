use crate::api_interface::posts::*;
use crate::models::nft::NftToken;

// Helper methods for generating requests
pub fn generate_create_post_request(
    created_ts: u64,
    words: String,
    created_by: String,
    nfts: Vec<NftToken>,
) -> CreatePostRequest {
    CreatePostRequest {
        post_id: created_ts.to_string(),
        created_by,
        nfts,
        in_public: true,
        club_id: "".to_string(),
        words,
        created_ts,
    }
}

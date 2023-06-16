use crate::api::constants::DEFAULT_PAGE_SIZE;
use crate::api::helpers;
use crate::api::helpers::get_page_from_btree;
use crate::api_interface::post_trending::*;
use crate::models::trending_post::TrendingPostKey;
use crate::models::trending_post_collection::TrendingPostCollectionKey;
use crate::stable_structure::access_helper::*;
use candid::candid_method;
use ic_cdk_macros::{query, update};

#[update]
#[candid_method(update)]
pub fn update_club_post_trending_score(request: UpdateClubPostStreetTrendingScoreRequest) {
    helpers::update_trending_club_post_indexes(&request.new, request.nft_canister_ids);
}

#[query]
#[candid_method(query)]
pub fn get_trending_street_posts(
    request: GetTrendingStreetPostRequest,
) -> GetTrendingStreetPostResponse {
    with_trending_posts_street(|max_heap| {
        let start = request.cursor.0.unwrap_or(TrendingPostKey {
            post_id: "".to_string(),
            trending_score: u32::MAX,
            created_ts: u64::MAX,
            updated_ts: u64::MAX,
            club_id: None,
        });
        let end = TrendingPostKey::lowest();
        let limit = request.limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize;

        let (posts, next_cursor) = get_page_from_btree(max_heap, start, end, limit);

        GetTrendingStreetPostResponse {
            posts,
            next_cursor,
            error: None,
        }
    })
}

#[query]
#[candid_method(query)]
pub fn get_trending_collection_posts(
    request: GetTrendingCollectionPostRequest,
) -> GetTrendingCollectionPostResponse {
    with_trending_posts_collection(|max_heap| {
        let start = request.cursor.0.unwrap_or(TrendingPostCollectionKey {
            canister_id: request.canister_id.clone(),
            trending_info: TrendingPostKey {
                post_id: "".to_string(),
                trending_score: u32::MAX,
                created_ts: u64::MAX,
                updated_ts: u64::MAX,
                club_id: None,
            },
        });
        let end = TrendingPostCollectionKey {
            canister_id: request.canister_id.clone(),
            trending_info: TrendingPostKey::lowest(),
        };
        let limit = request.limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize;

        let (posts, next_cursor) = get_page_from_btree(max_heap, start, end, limit);

        GetTrendingCollectionPostResponse {
            posts,
            next_cursor,
            error: None,
        }
    })
}

use candid::candid_method;
use ic_cdk_macros::query;

use crate::api::constants::DEFAULT_PAGE_SIZE;
use crate::stable_structure::access_helper::*;

use super::helpers_api::*;
use crate::api_interface::posts_interface::*;
use crate::models::post_user_model::UserPostCreatedTsKey;

#[query]
#[candid_method(query)]
pub fn get_posts_by_user(request: GetUserPostsRequest) -> GetUserPostsResponse {
    with_user_posts_created(|max_heap| {
        let start = request.cursor.0.unwrap_or(UserPostCreatedTsKey {
            user_id: request.user_id.clone(),
            // when first time, set to max created ts as the starting point
            created_ts: u64::MAX,
            post_id: "".to_string(),
            club_id: None,
        });
        let end = UserPostCreatedTsKey {
            user_id: request.user_id.clone(),
            created_ts: 0,
            post_id: "".to_string(),
            club_id: None,
        };
        let limit = request.limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize;

        let (posts, next_cursor) = get_page_from_btree(max_heap, start, end, limit);

        GetUserPostsResponse {
            posts,
            next_cursor,
            error: None,
        }
    })
}

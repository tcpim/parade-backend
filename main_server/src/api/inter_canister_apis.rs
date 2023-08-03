use candid::candid_method;
use ic_cdk_macros::update;

use crate::api_interface::common_interface::ServerError;
use crate::api_interface::inter_canister_interface::{
    AddClubPostToStreetRequest, AddClubPostToUserRequest, UpdateClubPostStreetTrendingScoreRequest,
};
use crate::stable_structure::access_helper::*;

use super::helpers_api::*;
use crate::models::post_user_model::UserPostCreatedTsKey;

use crate::models::post_collection_model::CollectionPostCreatedTsKey;

use crate::models::post_street_model::PostCreatedTsKey;

use crate::models::trending_post_collection_model::TrendingPostCollectionKey;
use crate::models::trending_post_model::TrendingPostKey;

/**
Add street/club post to the user storage
 */
#[update]
#[candid_method(update)]
pub fn add_club_post_to_user(request: AddClubPostToUserRequest) -> Option<ServerError> {
    if !is_inter_canister_caller_authorized(request.caller) {
        return Some(ServerError {
            api_name: "add_club_post_to_user".to_string(),
            error_message: format!("Unauthorized caller: {}", ic_cdk::caller().to_string()),
        });
    }

    let key = request.user_post_created_key;
    with_user_posts_created_mut(|max_heap| {
        max_heap.insert(
            UserPostCreatedTsKey {
                user_id: key.user_id.clone(),
                created_ts: key.created_ts,
                post_id: key.post_id.clone(),
                club_id: key.club_id.clone(),
            },
            (),
        );
    });

    None
}

/**
Add a club post to the street storage with post id and club id
1. Add to street storage and trending
2. If there is nft, add to collection storage and trending
 */
#[update]
#[candid_method(update)]
pub fn add_club_post_to_street(request: AddClubPostToStreetRequest) -> Option<ServerError> {
    if !is_inter_canister_caller_authorized(request.caller) {
        return Some(ServerError {
            api_name: "add_club_post_to_street".to_string(),
            error_message: format!("Unauthorized caller: {}", ic_cdk::caller().to_string()),
        });
    }

    with_street_posts_created_mut(|max_heap| {
        max_heap.insert(
            PostCreatedTsKey {
                post_id: request.post_id.clone(),
                created_ts: request.created_ts,
                club_id: Some(request.club_id.clone()),
            },
            (),
        );
    });

    let trending_key = TrendingPostKey {
        post_id: request.post_id.clone(),
        created_ts: request.created_ts,
        updated_ts: request.created_ts,
        trending_score: 0,
        club_id: Some(request.club_id.clone()),
    };
    with_trending_posts_street_mut(|trending_posts_street| {
        trending_posts_street.insert(trending_key.clone(), ());
    });

    if !request.nfts.is_empty() {
        // currently only support one NFT per post
        let canister_id = request.nfts[0].clone().canister_id;

        with_collection_posts_created_mut(|max_heap| {
            max_heap.insert(
                CollectionPostCreatedTsKey {
                    canister_id: canister_id.clone(),
                    post_id: request.post_id.clone(),
                    created_ts: request.created_ts,
                    club_id: Some(request.club_id.clone()),
                },
                (),
            );
        });

        with_trending_posts_collection_mut(|max_heap| {
            max_heap.insert(
                TrendingPostCollectionKey {
                    canister_id: canister_id.clone(),
                    trending_info: trending_key.clone(),
                },
                (),
            );
        });
    };

    None
}

#[update]
#[candid_method(update)]
pub fn update_club_post_trending_score(request: UpdateClubPostStreetTrendingScoreRequest) {
    if !is_inter_canister_caller_authorized(request.caller) {
        return;
    }

    update_trending_club_post_indexes(&request.new, request.nft_canister_ids);
}

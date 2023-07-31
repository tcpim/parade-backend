use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::api::constants::DEFAULT_PAGE_SIZE;
use crate::stable_structure::access_helper::*;
use std::collections::BTreeMap;

use super::helpers_api::*;
use crate::api_interface::common_interface::*;
use crate::api_interface::posts_interface::*;
use crate::models::post_collection_model::CollectionPostCreatedTsKey;
use crate::models::post_model::*;
use crate::models::post_street_model::PostCreatedTsKey;
use crate::models::post_user_model::UserPostCreatedTsKey;
use crate::models::trending_post_collection_model::TrendingPostCollectionKey;

// ######################
// APIs
// ######################

/**
Create a new street post
1. Add post to post by id.
2. Add post to street post and street trending
3. If this post contains nfts, add post to posts_by_collection and trending posts by collection
4. Add to user post storage
 */
#[update]
#[candid_method(update)]
pub fn create_street_post(request: CreateStreetPostRequest) -> CreateStreetPostResponse {
    let post_id = PostIdString(request.post_id);

    let post = Post {
        id: post_id.clone(),
        created_by: request.created_by.clone(),
        nfts: request.nfts,
        words: request.words,
        created_ts: request.created_ts,
        updated_ts: request.created_ts,
        replies: vec![],
        emoji_reactions: BTreeMap::new(),
    };

    if !is_caller_authorized() {
        return CreateStreetPostResponse {
            post: post,
            error: Some(ServerError {
                api_name: "create_street_post".to_string(),
                error_message: "Unauthorized".to_string(),
            }),
        };
    }

    let mut error: Option<ServerError> = None;

    // Add post to post by id
    with_post_by_id_mut(|post_by_id| {
        match post_by_id.get(&post_id) {
            Some(_) => {
                error = Some(ServerError {
                    api_name: "create_street_post".to_string(),
                    error_message: "Post already exists".to_string(),
                });
            }
            None => {
                post_by_id.insert(post_id.clone(), post.clone());
            }
        };
    });
    if error.is_some() {
        return CreateStreetPostResponse { post, error };
    }

    // Add post to street post and trending
    with_street_posts_created_mut(|max_heap| {
        max_heap.insert(
            PostCreatedTsKey {
                post_id: post.id.0.clone(),
                created_ts: post.created_ts,
                club_id: None,
            },
            (),
        );
    });

    with_trending_posts_street_mut(|trending_posts_street| {
        trending_posts_street.insert(get_trending_post_key(&post, None), ());
    });

    // Add posts to posts by collection and trending
    if !post.nfts.is_empty() {
        // currently only support one NFT per post
        let canister_id = post.nfts[0].clone().canister_id;

        with_collection_posts_created_mut(|max_heap| {
            max_heap.insert(
                CollectionPostCreatedTsKey {
                    canister_id: canister_id.clone(),
                    post_id: post.id.0.clone(),
                    created_ts: post.created_ts,
                    club_id: None,
                },
                (),
            );
        });

        with_trending_posts_collection_mut(|max_heap| {
            max_heap.insert(
                TrendingPostCollectionKey {
                    canister_id: canister_id.clone(),
                    trending_info: get_trending_post_key(&post, None),
                },
                (),
            );
        });
    }

    with_user_posts_created_mut(|max_heap| {
        max_heap.insert(
            UserPostCreatedTsKey {
                user_id: post.created_by.clone(),
                post_id: post.id.0.clone(),
                created_ts: post.created_ts,
                club_id: None,
            },
            (),
        );
    });

    CreateStreetPostResponse { post, error }
}

#[query]
#[candid_method(query)]
pub fn get_street_posts(request: GetStreetPostsRequest) -> GetStreetPostsResponse {
    with_street_posts_created(|max_heap| {
        let start = request.cursor.0.unwrap_or(PostCreatedTsKey {
            // when first time, set to max created ts as the starting point
            created_ts: u64::MAX,
            post_id: "".to_string(),
            club_id: None,
        });
        let end = PostCreatedTsKey {
            created_ts: 0,
            post_id: "".to_string(),
            club_id: None,
        };
        let limit = request.limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize;

        let (posts, next_cursor) = get_page_from_btree(max_heap, start, end, limit);
        GetStreetPostsResponse {
            posts,
            next_cursor,
            error: None,
        }
    })
}

#[query]
#[candid_method(query)]
pub fn get_posts_by_collection(request: GetCollectionPostsRequest) -> GetCollectionPostsResponse {
    with_collection_posts_created(|max_heap| {
        let start = request.cursor.0.unwrap_or(CollectionPostCreatedTsKey {
            canister_id: request.canister_id.clone(),
            // when first time, set to max created ts as the starting point
            created_ts: u64::MAX,
            post_id: "".to_string(),
            club_id: None,
        });
        let end = CollectionPostCreatedTsKey {
            canister_id: request.canister_id.clone(),
            created_ts: 0,
            post_id: "".to_string(),
            club_id: None,
        };
        let limit = request.limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize;

        let (posts, next_cursor) = get_page_from_btree(max_heap, start, end, limit);

        GetCollectionPostsResponse {
            posts,
            next_cursor,
            error: None,
        }
    })
}

#[query]
#[candid_method(query)]
pub fn get_street_post_by_id(post_id: String) -> GetPostByIdResponse {
    let post = get_post_by_id_from_store(&PostIdString(post_id));
    if post.is_none() {
        GetPostByIdResponse { post: None }
    } else {
        GetPostByIdResponse { post }
    }
}

use candid::{candid_method, Principal};
use ic_cdk_macros::{query, update};

use crate::api::constants::{DEFAULT_PAGE_SIZE, MAIN_SERVER_CANISTER_ID};
use crate::stable_structure::access_helper::*;
use std::collections::BTreeMap;

use super::helpers::*;
use crate::api_interface::common::*;
use crate::api_interface::posts::*;
use crate::models::post::PostCreatedTsKey;
use crate::models::post::*;
use crate::models::post_collection::CollectionPostCreatedTsKey;
use crate::models::trending_post_collection::TrendingPostCollectionKey;

use crate::api_interface::inter_canister::{
    AddClubPostToStreetRequest, UserPostCreatedTsKeyExternal,
};

// ######################
// APIs
// ######################

/**
Create a new post
1. Add post to post by id
2. Add post to post by club and trending posts by club
3. If this post contains nfts, add post to posts_by_collection and trending posts by collection
4. If this post is public, add to main server storage
5. Add to user post storage
 */
#[update]
#[candid_method(update)]
pub async fn create_post(request: CreatePostRequest) -> CreatePostResponse {
    let post_id = PostIdString(request.post_id.clone());
    let user = request.created_by.clone();

    let post = Post {
        id: post_id.clone(),
        club_id: get_club_id(),
        created_by: user,
        nfts: request.nfts.clone(),
        in_public: request.in_public,
        words: request.words,
        created_ts: request.created_ts,
        updated_ts: request.created_ts,
        replies: vec![],
        emoji_reactions: BTreeMap::new(),
    };

    if request.post_id.is_empty() {
        return CreatePostResponse {
            post,
            error: Some(ServerError::CreatePostGeneralError(
                "Post id cannot be empty".to_string(),
            )),
        };
    }

    let mut error: Option<ServerError> = None;
    // Add post to post by id
    with_post_by_id_mut(|post_by_id| {
        match post_by_id.get(&post_id) {
            Some(_) => {
                error = Some(ServerError::CreatePostGeneralError(
                    "Post already exists".to_string(),
                ));
            }
            None => {
                post_by_id.insert(post_id.clone(), post.clone());
            }
        };
    });
    if error.is_some() {
        return CreatePostResponse { post, error };
    }

    with_club_posts_created_mut(|max_heap| {
        max_heap.insert(
            PostCreatedTsKey {
                post_id: post.id.0.clone(),
                created_ts: post.created_ts,
            },
            (),
        );
    });

    with_trending_posts_mut(|max_heap| {
        max_heap.insert(get_trending_post_key(&post), ());
    });

    // Add posts to posts by collection
    if !post.nfts.is_empty() {
        // currently only support one NFT per post
        let canister_id = post.nfts[0].clone().canister_id;

        with_collection_posts_created_mut(|max_heap| {
            max_heap.insert(
                CollectionPostCreatedTsKey {
                    canister_id: canister_id.clone(),
                    post_id: post.id.0.clone(),
                    created_ts: post.created_ts,
                },
                (),
            );
        });

        with_trending_posts_collection_mut(|max_heap| {
            max_heap.insert(
                TrendingPostCollectionKey {
                    canister_id: canister_id.clone(),
                    trending_info: get_trending_post_key(&post),
                },
                (),
            );
        });
    }

    call_inter_canister_async(
        MAIN_SERVER_CANISTER_ID,
        "add_club_post_to_user",
        UserPostCreatedTsKeyExternal {
            user_id: request.created_by.clone(),
            post_id: request.post_id.clone(),
            club_id: Some(get_club_id()),
            created_ts: request.created_ts,
        },
        "Failed to add post to user storage",
    )
    .await;

    if post.in_public {
        call_inter_canister_async(
            MAIN_SERVER_CANISTER_ID,
            "add_club_post_to_street",
            AddClubPostToStreetRequest {
                post_id: request.post_id.clone(),
                club_id: get_club_id(),
                nfts: convert_to_main_server_nfttoken(request.nfts.clone()),
                created_ts: request.created_ts,
                created_by: request.created_by.clone(),
            },
            "Failed to add post to street storage",
        )
        .await;
    }

    CreatePostResponse { post, error }
}

#[query]
#[candid_method(query)]
pub fn get_posts(request: GetPostsRequest) -> GetPostsResponse {
    with_club_posts_created(|max_heap| {
        let start = request.cursor.0.unwrap_or(PostCreatedTsKey {
            // when first time, set to max created ts as the starting point
            created_ts: u64::MAX,
            post_id: "".to_string(),
        });
        let end = PostCreatedTsKey {
            created_ts: 0,
            post_id: "".to_string(),
        };
        let limit = request.limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize;

        let (posts, next_cursor) = get_page_from_btree(max_heap, start, end, limit);

        GetPostsResponse {
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
        });
        let end = CollectionPostCreatedTsKey {
            canister_id: request.canister_id.clone(),
            created_ts: 0,
            post_id: "".to_string(),
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
pub fn get_post_by_id(post_id: String) -> GetPostByIdResponse {
    let post = get_post_by_id_from_store(&PostIdString(post_id.clone()));
    if post.is_none() {
        GetPostByIdResponse { post: None }
    } else {
        GetPostByIdResponse { post }
    }
}

#[query]
#[candid_method(query)]
pub fn get_post_by_ids(post_id: Vec<String>) -> GetPostByIdsResponse {
    let mut posts: Vec<Option<Post>> = vec![];
    for id in post_id {
        let post = get_post_by_id_from_store(&PostIdString(id.clone()));
        posts.push(post);
    }

    GetPostByIdsResponse { posts }
}

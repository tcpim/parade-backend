use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::api::helpers_api::{
    call_inter_canister, get_caller, get_club_id, get_post_by_id_from_store, is_caller_authorized,
};
use crate::stable_structure::access_helper::*;
use std::collections::BTreeMap;

use super::helpers_api;
use crate::api_interface::common_interface::*;
use crate::api_interface::inter_canister_interface::{
    TrendingPostKeyExternal, UpdateClubPostStreetTrendingScoreRequest,
};
use crate::api_interface::post_reply_interface::*;
use crate::models::post_model::*;

/**
Add a new reply to a post
1. Add the reply to the reply by id store
2. Modify the existing post with newly added reply and newly updated_ts
3. Get a new post trending score
    a. update the post_by_id store for the stored trending score
    b. update the trending storages trending and trending_collection
    c. update the trending score in the main server
*/
#[update]
#[candid_method(update)]
pub async fn reply_post(request: ReplyPostRequest) -> ReplyPostResponse {
    let mut error = None;
    let caller = get_caller();
    let post_reply_string_id = PostReplyIdString(request.reply_id);
    let post_reply = PostReply {
        id: post_reply_string_id.clone(),
        post_id: PostIdString(request.post_id.clone()),
        created_by: request.user.clone(),
        words: request.words.clone(),
        created_ts: request.created_ts,
        nfts: request.nfts.clone(),
        emoji_reactions: BTreeMap::new(),
    };

    if !is_caller_authorized() {
        return ReplyPostResponse {
            reply: post_reply,
            error: Some(ServerError {
                api_name: "reply_post".to_string(),
                error_message: format!("Unauthorized caller: {}", ic_cdk::caller().to_string()),
            }),
        };
    }

    // Fake initial data
    let mut post_new: Post = Post {
        id: PostIdString("".to_string()),
        club_id: get_club_id(),
        created_by: "".to_string(),
        nfts: vec![],
        in_public: false,
        words: "".to_string(),
        created_ts: 0,
        updated_ts: 0,
        replies: vec![],
        emoji_reactions: Default::default(),
    };

    with_post_by_id_mut(|storage| {
        // Get post
        let post_opt = storage.get(&PostIdString(request.post_id.clone()));
        if post_opt.is_none() {
            error = Some(ServerError {
                api_name: "reply_post".to_string(),
                error_message: format!("Failed to find post by id when reply: {}", request.post_id),
            });
        }

        let post = post_opt.unwrap();

        // add reply to reply id -> reply store
        let reply_already_exists_err =
            helpers_api::add_post_reply_to_reply_store(post_reply.clone());
        if reply_already_exists_err.is_some() {
            error = reply_already_exists_err;
        }

        // Update post content
        post_new = post.clone();
        post_new.updated_ts = request.created_ts;
        post_new.replies.push(post_reply_string_id);
        let new_trending_post_key = helpers_api::get_trending_post_key(&post_new);
        storage.insert(post.id.clone(), post_new.clone());

        // Update trending score btree indexes
        helpers_api::update_trending_post_indexes(post, &new_trending_post_key);
    });

    if post_new.in_public {
        let new_trending_post_key = helpers_api::get_trending_post_key(&post_new);
        call_inter_canister(
            "update_club_post_trending_score",
            UpdateClubPostStreetTrendingScoreRequest {
                new: TrendingPostKeyExternal {
                    post_id: new_trending_post_key.post_id,
                    trending_score: new_trending_post_key.trending_score,
                    created_ts: new_trending_post_key.created_ts,
                    updated_ts: new_trending_post_key.updated_ts,
                    club_id: Some(get_club_id()),
                },
                nft_canister_ids: post_new.nfts.into_iter().map(|x| x.canister_id).collect(),
                caller: caller.clone(),
            },
            "update_club_post_trending_score failed",
        )
        .await;
    }

    ReplyPostResponse {
        error,
        reply: post_reply,
    }
}

#[query]
#[candid_method(query)]
pub fn get_post_replies(request: GetPostRepliesRequest) -> GetPostRepliesResponse {
    let mut result_vec: Vec<PostReply> = vec![];
    let mut result_offset = 0;
    let mut error = None;

    let post = get_post_by_id_from_store(&PostIdString(request.post_id.clone()));
    if post.is_none() {
        error = Some(ServerError {
            api_name: "get_post_replies".to_string(),
            error_message: format!(
                "Failed to find post in POST_BY_ID with post_id: {:?}",
                &request.post_id
            ),
        });
    } else if let Some(post) = post {
        let reply_ids = post.replies;
        let (result_post_replies, next_offset) =
            helpers_api::get_post_replies_from_reply_ids(reply_ids, request.offset, request.limit);
        result_vec = result_post_replies;
        result_offset = next_offset;
    }

    GetPostRepliesResponse {
        offset: result_offset,
        post_replies: result_vec,
        error,
    }
}

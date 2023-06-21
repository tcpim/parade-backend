use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::api::helpers_api::get_post_by_id_from_store;
use crate::stable_structure::access_helper::*;
use std::collections::BTreeMap;

use super::helpers_api;
use crate::api_interface::common_interface::*;
use crate::api_interface::post_reply_interface::*;
use crate::models::post_model::*;

/**
Add a new reply to a post
1. Add the reply to the reply by id store
2. Modify the existing post with newly added reply and newly updated_ts
3. Get a new post trending score
    a. update the post_by_id store for the stored trending score
    b. update the trending storages such as trending_club and trending_collection
*/
#[update]
#[candid_method(update)]
pub fn reply_post(request: ReplyPostRequest) -> ReplyPostResponse {
    let mut error = None;
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
    with_post_by_id_mut(|storage| {
        // Get post
        let post_opt = storage.get(&PostIdString(request.post_id.clone()));
        if post_opt.is_none() {
            error = Some(ServerError::ReplyPostError(format!(
                "Should not happen! Failed to find post by id when reply: {}",
                request.post_id
            )));
        }
        let post = post_opt.unwrap();

        // add reply to reply id -> reply store
        let reply_already_exists_err =
            helpers_api::add_post_reply_to_reply_store(post_reply.clone());
        if reply_already_exists_err.is_some() {
            error = reply_already_exists_err;
        }

        // Update post content
        let mut new_post = post.clone();
        new_post.updated_ts = request.created_ts;
        new_post.replies.push(post_reply_string_id);
        storage.insert(post.id.clone(), new_post.clone());

        // Update trending score btree indexes
        helpers_api::update_trending_post_indexes(&new_post, None);
    });

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
        error = Some(ServerError::GetPostRepliesError(format!(
            "Should not happen Failed to find post in POST_BY_ID with post_id: {:?}",
            &request.post_id
        )));
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

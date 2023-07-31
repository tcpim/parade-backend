use crate::api::helpers_api;
use crate::api::helpers_api::is_caller_authorized;
use crate::stable_structure::access_helper::*;
use candid::candid_method;
use ic_cdk_macros::update;

use crate::api_interface::common_interface::*;
use crate::api_interface::post_reaction_interface::*;
use crate::models::post_model::*;

#[update]
#[candid_method(update)]
pub fn react_emoji(request: ReactEmojiRequest) -> ReactEmojiResponse {
    if !is_caller_authorized() {
        return ReactEmojiResponse {
            error: Some(ServerError {
                api_name: "react_emoji".to_string(),
                error_message: "caller not authorized".to_string(),
            }),
        };
    }

    let mut error = None;

    // This reaction is for a post
    if request.post_id.is_some() {
        let post_id_string = PostIdString(request.post_id.clone().unwrap());
        with_post_by_id_mut(|storage| {
            if storage.get(&post_id_string).is_none() {
                error = Some(ServerError {
                    api_name: "react_emoji".to_string(),
                    error_message: format!(
                        "Failed to find post in POST_BY_ID with post_id: {:?}",
                        &post_id_string
                    ),
                })
            }

            let post = storage.get(&post_id_string).unwrap();
            let mut emojis = post.emoji_reactions.clone();
            match emojis.get(&request.emoji) {
                Some(count) => {
                    emojis.insert(request.emoji.clone(), count + 1);
                }
                None => {
                    emojis.insert(request.emoji.clone(), 1);
                }
            }

            // Update post content
            let mut new_post = post;
            new_post.updated_ts = request.created_ts;
            new_post.emoji_reactions = emojis;
            storage.insert(post_id_string.clone(), new_post.clone());

            // Update trending score in btrees
            helpers_api::update_trending_post_indexes(&new_post, None);
        })
    }

    if request.reply_id.is_some() {
        let post_reply_id_string = PostReplyIdString(request.reply_id.clone().unwrap());
        with_post_reply_by_id_mut(|post_reply_by_id| {
            if post_reply_by_id.get(&post_reply_id_string).is_none() {
                error = Some(ServerError {
                    api_name: "react_emoji".to_string(),
                    error_message: format!(
                        "Failed to find post reply in POST_REPLY_BY_ID with post_reply_id: {:?}",
                        &post_reply_id_string
                    ),
                })
            }

            // Update post content
            let post_reply = post_reply_by_id.get(&post_reply_id_string).unwrap();
            let mut emojis = post_reply.emoji_reactions.clone();
            match emojis.get(&request.emoji) {
                Some(count) => {
                    emojis.insert(request.emoji.clone(), count + 1);
                }
                None => {
                    emojis.insert(request.emoji.clone(), 1);
                }
            };
            let mut post_reply_new = post_reply;
            post_reply_new.emoji_reactions = emojis;
            post_reply_by_id.insert(post_reply_id_string.clone(), post_reply_new);
        });
    }

    ReactEmojiResponse { error }
}

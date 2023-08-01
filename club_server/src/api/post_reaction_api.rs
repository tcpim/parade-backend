use crate::api::constants::MAIN_SERVER_CANISTER_ID;
use crate::api::helpers_api;
use crate::api::helpers_api::{
    call_inter_canister_async, get_caller_when_within_canister, get_club_id, is_caller_authorized,
};
use crate::stable_structure::access_helper::*;
use candid::candid_method;
use ic_cdk_macros::update;

use crate::api_interface::common_interface::*;
use crate::api_interface::inter_canister_interface::{
    TrendingPostKeyExternal, UpdateClubPostStreetTrendingScoreRequest,
};
use crate::api_interface::post_reaction_interface::*;
use crate::models::post_model::*;

/**
Add a new emoji to a post
1. Modify the existing post with newly added reply and newly updated_ts
2. Get a new post trending score
    a. update the post_by_id store for the stored trending score
    b. update the trending storages trending and trending_collection
    c. update the trending score in the main server
 */
#[update]
#[candid_method(update)]
pub async fn react_emoji(request: ReactEmojiRequest) -> ReactEmojiResponse {
    let caller = get_caller_when_within_canister();
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
            post_new = post.clone();
            post_new.updated_ts = request.created_ts;
            post_new.emoji_reactions = emojis;
            let new_trending_post_key = helpers_api::get_trending_post_key(&post_new);
            storage.insert(post_id_string.clone(), post_new.clone());

            // Update trending score in btrees
            helpers_api::update_trending_post_indexes(post, &new_trending_post_key);
        });

        if post_new.in_public {
            let new_trending_post_key = helpers_api::get_trending_post_key(&post_new);
            call_inter_canister_async(
                MAIN_SERVER_CANISTER_ID,
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

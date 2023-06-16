use crate::api::constants::{CLUB_ID, MAIN_SERVER_CANISTER_ID};
use crate::api::helpers;
use crate::api::helpers::call_inter_canister_async;
use crate::stable_structure::access_helper::*;
use candid::candid_method;
use ic_cdk_macros::update;

use crate::api_interface::common::*;
use crate::api_interface::inter_canister::{
    TrendingPostKeyExternal, UpdateClubPostStreetTrendingScoreRequest,
};
use crate::api_interface::post_reaction::*;
use crate::models::post::*;
use crate::models::trending_post::TrendingPostKey;

#[update]
#[candid_method(update)]
pub async fn react_emoji(request: ReactEmojiRequest) -> ReactEmojiResponse {
    let mut error = None;

    // This reaction is for a post
    if request.post_id.is_some() {
        let post_id_string = PostIdString(request.post_id.clone().unwrap());
        // Fake initial data
        let mut post_new: Post = Post {
            id: PostIdString("".to_string()),
            created_by: "".to_string(),
            nfts: vec![],
            in_public: false,
            words: "".to_string(),
            created_ts: 0,
            updated_ts: 0,
            replies: vec![],
            emoji_reactions: Default::default(),
            trending_score: None,
        };

        with_post_by_id_mut(|storage| {
            if storage.get(&post_id_string).is_none() {
                error = Some(ServerError::ReactEmojiError(format!(
                    "Should not happen due to post uuid. Failed to find post in POST_BY_ID with post_id: {:?}",
                    &post_id_string
                )))
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
            let new_trending_post_key = helpers::get_trending_post_key(&post_new);
            post_new.trending_score = Some(new_trending_post_key.trending_score);
            storage.insert(post_id_string.clone(), post_new.clone());

            // Update trending score in btrees
            helpers::update_trending_post_indexes(post, &new_trending_post_key);
        });

        if post_new.in_public {
            let new_trending_post_key = helpers::get_trending_post_key(&post_new);
            call_inter_canister_async(
                MAIN_SERVER_CANISTER_ID,
                "update_club_post_trending_score",
                UpdateClubPostStreetTrendingScoreRequest {
                    new: TrendingPostKeyExternal {
                        post_id: new_trending_post_key.post_id,
                        trending_score: new_trending_post_key.trending_score,
                        created_ts: new_trending_post_key.created_ts,
                        updated_ts: new_trending_post_key.updated_ts,
                        club_id: Some(CLUB_ID.to_string()),
                    },
                    nft_canister_ids: post_new.nfts.into_iter().map(|x| x.canister_id).collect(),
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
                error = Some(ServerError::ReactEmojiError(format!(
                    "Should not happen due to reply uuid. Failed to find post reply in POST_REPLY_BY_ID with post_reply_id: {:?}",
                    &post_reply_id_string
                )))
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

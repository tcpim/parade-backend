use super::helpers::*;
use crate::api::post::*;
use crate::api::post_reaction::*;
use crate::api::post_reply::*;
use crate::api_interface::post_reaction::*;
use crate::api_interface::post_reply::*;
use async_std::task;

#[test]
fn react_emoji_to_post_and_its_reply() {
    let post_res = task::block_on(create_post(generate_create_post_request(
        0,
        "hi_1".to_string(),
        "tim".to_string(),
        vec![],
    )));
    let old_post = get_post_by_id(post_res.post.id.0.clone()).post;

    let reply_res = task::block_on(reply_post(ReplyPostRequest {
        reply_id: "1".to_string(),
        user: "peter".to_string(),
        post_id: post_res.post.id.0.clone(),
        nfts: vec![],
        created_ts: 1,
        words: "reply to tim!".to_string(),
    }));

    let post_id = post_res.post.id.0.clone();
    let reply_id = reply_res.reply.id.0.clone();

    task::block_on(react_emoji(ReactEmojiRequest {
        post_id: Some(post_id.clone()),
        reply_id: None,
        emoji: "👍".to_string(),
        user: "ryan".to_string(),
        created_ts: 2,
    }));
    task::block_on(react_emoji(ReactEmojiRequest {
        post_id: None,
        reply_id: Some(reply_id.clone()),
        emoji: "🤣".to_string(),
        user: "ryan".to_string(),
        created_ts: 3,
    }));

    let new_post = get_post_by_id(post_id.clone()).post;
    assert_eq!(new_post.len(), 1);
    assert_eq!(new_post[0].emoji_reactions.get("👍").unwrap(), &1);

    let reply = get_post_replies(GetPostRepliesRequest {
        post_id: post_id.clone(),
        offset: 0,
        limit: None,
    })
    .post_replies;

    assert_eq!(reply[0].emoji_reactions.len(), 1);
    assert_eq!(reply[0].emoji_reactions.get("🤣").unwrap(), &1);

    // After all reactions, assert the trending score changed
    assert_ne!(old_post[0].trending_score, new_post[0].trending_score);
}

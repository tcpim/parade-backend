use super::helpers::*;
use crate::api::post_api::*;
use crate::api::post_reaction_api::react_emoji;
use crate::api::post_reply_api::reply_post;
use crate::api::post_trending_api::{get_trending_collection_posts, get_trending_posts};
use crate::api_interface::common_interface::Cursor;
use crate::api_interface::post_reaction_interface::ReactEmojiRequest;
use crate::api_interface::post_reply_interface::ReplyPostRequest;
use crate::api_interface::post_trending_interface::{
    GetTrendingCollectionPostRequest, GetTrendingPostRequest,
};
use crate::models::nft_model::NftToken;
use async_std::task;

#[test]
fn get_trending_posts_pagination() {
    // Set up
    // 3 posts, trending order: post2, post3, post1
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), vec![]);
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "tim".to_string(), vec![]);
    let create_post_request_3 =
        generate_create_post_request(2, "hi_3".to_string(), "tim".to_string(), vec![]);
    let post1 = task::block_on(create_post(create_post_request_1));
    let post2 = task::block_on(create_post(create_post_request_2));
    let post3 = task::block_on(create_post(create_post_request_3));

    // trending list: 2, 3, 1
    make_posts_trending(&post2.post.id.0, &post3.post.id.0, &post1.post.id.0);

    // Act
    let response = get_trending_posts(GetTrendingPostRequest {
        limit: Option::Some(1),
        cursor: Cursor(None),
    });

    // Assert
    // get the most popular post
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.posts[0].words, "hi_2");
    assert_ne!(response.next_cursor, Cursor(None));

    // get second page
    let response = get_trending_posts(GetTrendingPostRequest {
        limit: Option::Some(2),
        cursor: response.next_cursor,
    });
    assert_eq!(response.posts.len(), 2);
    assert_eq!(response.posts[0].words, "hi_3");
    assert_eq!(response.posts[1].words, "hi_1");
    assert_eq!(response.next_cursor, Cursor(None));
}

#[test]
fn get_trending_collection_posts_pagination() {
    // Set up
    let nfts = vec![NftToken {
        canister_id: "canister_1".to_string(),
        token_index: 1,
        token_id: "".to_string(),
        collection_name: "".to_string(),
        original_image_url: "".to_string(),
        original_thumbnail_url: "".to_string(),
    }];
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), nfts.clone());
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "tim".to_string(), nfts.clone());
    let create_post_request_3 =
        generate_create_post_request(2, "hi_3".to_string(), "tim".to_string(), nfts.clone());
    let post1 = task::block_on(create_post(create_post_request_1));
    let post2 = task::block_on(create_post(create_post_request_2));
    let post3 = task::block_on(create_post(create_post_request_3));

    // trending list: 2, 3, 1
    make_posts_trending(&post2.post.id.0, &post3.post.id.0, &post1.post.id.0);

    // Act
    let response = get_trending_collection_posts(GetTrendingCollectionPostRequest {
        canister_id: "canister_1".to_string(),
        limit: Option::Some(1),
        cursor: Cursor(None),
    });

    // Assert
    // get the most popular post
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.posts[0].words, "hi_2");
    assert_ne!(response.next_cursor, Cursor(None));

    // get second page
    let response = get_trending_collection_posts(GetTrendingCollectionPostRequest {
        canister_id: "canister_1".to_string(),
        limit: Option::Some(2),
        cursor: response.next_cursor,
    });
    assert_eq!(response.posts.len(), 2);
    assert_eq!(response.posts[0].words, "hi_3");
    assert_eq!(response.posts[1].words, "hi_1");
    assert_eq!(response.next_cursor, Cursor(None));
}

/**
1. 2 reply and 1 emoji to first post
2. 1 reply and 1 emoji to second post
3. 1 emoji to third post
*/
fn make_posts_trending(first: &String, second: &String, third: &String) {
    // 2 reply and 1 emoji to first post
    task::block_on(reply_post(ReplyPostRequest {
        reply_id: "1".to_string(),
        user: "peter".to_string(),
        post_id: first.clone(),
        nfts: vec![],
        created_ts: 1,
        words: "this is 1st reply".to_string(),
    }));
    task::block_on(reply_post(ReplyPostRequest {
        reply_id: "2".to_string(),
        user: "peter".to_string(),
        post_id: first.clone(),
        nfts: vec![],
        created_ts: 2,
        words: "this is 2nd reply".to_string(),
    }));
    task::block_on(react_emoji(ReactEmojiRequest {
        post_id: Some(first.clone()),
        reply_id: None,
        emoji: "👍".to_string(),
        user: "ryan".to_string(),
        created_ts: 2,
    }));

    // 1 reply and 1 emoji to second post
    task::block_on(reply_post(ReplyPostRequest {
        reply_id: "3".to_string(),
        user: "peter".to_string(),
        post_id: second.clone(),
        nfts: vec![],
        created_ts: 2,
        words: "this is 1st reply".to_string(),
    }));
    task::block_on(react_emoji(ReactEmojiRequest {
        post_id: Some(second.clone()),
        reply_id: None,
        emoji: "👍".to_string(),
        user: "ryan".to_string(),
        created_ts: 2,
    }));

    // 1 emoji to third post
    task::block_on(react_emoji(ReactEmojiRequest {
        post_id: Some(third.clone()),
        reply_id: None,
        emoji: "👍".to_string(),
        user: "ryan".to_string(),
        created_ts: 2,
    }));
}

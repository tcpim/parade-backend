use super::helpers::*;
use crate::api::post_api::*;
use crate::api::post_reaction_api::react_emoji;
use crate::api::post_reply_api::reply_post;
use crate::api::post_trending_api::{
    get_trending_collection_posts, get_trending_street_posts, update_club_post_trending_score,
};
use crate::api_interface::common_interface::Cursor;
use crate::api_interface::post_reaction_interface::ReactEmojiRequest;
use crate::api_interface::post_reply_interface::ReplyPostRequest;
use crate::api_interface::post_trending_interface::{
    GetTrendingCollectionPostRequest, GetTrendingStreetPostRequest,
    UpdateClubPostStreetTrendingScoreRequest,
};
use crate::api_interface::posts_interface::AddClubPostToStreetRequest;
use crate::models::nft_model::NftToken;
use crate::models::post_model::{Post, PostIdString, PostReplyIdString};
use crate::models::trending_post_model::TrendingPostKey;

#[test]
fn get_trending_street_posts_pagination() {
    // Set up
    // 3 posts, trending order: post2, post3, post1
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), vec![]);
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "tim".to_string(), vec![]);
    let create_post_request_3 =
        generate_create_post_request(2, "hi_3".to_string(), "tim".to_string(), vec![]);
    let post1 = create_street_post(create_post_request_1);
    let post2 = create_street_post(create_post_request_2);
    let post3 = create_street_post(create_post_request_3);

    // trending list: 2, 3, 1
    make_posts_trending(&post2.post.id.0, &post3.post.id.0, &post1.post.id.0);

    // Act
    let response = get_trending_street_posts(GetTrendingStreetPostRequest {
        limit: Option::Some(1),
        cursor: Cursor(None),
    });

    // Assert
    // get the most popular post
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.posts[0].post.clone().unwrap().words, "hi_2");
    assert_ne!(response.next_cursor, Cursor(None));

    // get second page
    let response = get_trending_street_posts(GetTrendingStreetPostRequest {
        limit: Option::Some(2),
        cursor: response.next_cursor,
    });
    assert_eq!(response.posts.len(), 2);
    assert_eq!(response.posts[0].post.clone().unwrap().words, "hi_3");
    assert_eq!(response.posts[1].post.clone().unwrap().words, "hi_1");
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
        image_url: "".to_string(),
        image_thumbnail_url: "".to_string(),
    }];
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), nfts.clone());
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "tim".to_string(), nfts.clone());
    let create_post_request_3 =
        generate_create_post_request(2, "hi_3".to_string(), "tim".to_string(), nfts.clone());
    let post1 = create_street_post(create_post_request_1);
    let post2 = create_street_post(create_post_request_2);
    let post3 = create_street_post(create_post_request_3);

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
    assert_eq!(response.posts[0].post.clone().unwrap().words, "hi_2");
    assert_ne!(response.next_cursor, Cursor(None));

    // get second page
    let response = get_trending_collection_posts(GetTrendingCollectionPostRequest {
        canister_id: "canister_1".to_string(),
        limit: Option::Some(2),
        cursor: response.next_cursor,
    });
    assert_eq!(response.posts.len(), 2);
    assert_eq!(response.posts[0].post.clone().unwrap().words, "hi_3");
    assert_eq!(response.posts[1].post.clone().unwrap().words, "hi_1");
    assert_eq!(response.next_cursor, Cursor(None));
}

#[test]
fn update_club_post_street_trending_score() {
    // set up
    // Add 2 club post to street
    let nfts = vec![NftToken {
        canister_id: "canister_1".to_string(),
        token_index: 1,
        token_id: "".to_string(),
        collection_name: "".to_string(),
        image_url: "".to_string(),
        image_thumbnail_url: "".to_string(),
    }];
    let request_1 = AddClubPostToStreetRequest {
        post_id: "1".to_string(),
        club_id: "club_1".to_string(),
        nfts: nfts.clone(),
        created_ts: 1,
        created_by: "tim".to_string(),
    };
    let request_2 = AddClubPostToStreetRequest {
        post_id: "2".to_string(),
        club_id: "club_1".to_string(),
        nfts: nfts.clone(),
        created_ts: 2,
        created_by: "tim".to_string(),
    };
    add_club_post_to_street(request_1);
    add_club_post_to_street(request_2);

    // Act
    let street_response_before = get_trending_street_posts(GetTrendingStreetPostRequest {
        limit: None,
        cursor: Cursor(None),
    });
    let collection_response_before =
        get_trending_collection_posts(GetTrendingCollectionPostRequest {
            canister_id: "canister_1".to_string(),
            limit: None,
            cursor: Cursor(None),
        });
    // Update trending score of the first post
    update_club_post_trending_score(UpdateClubPostStreetTrendingScoreRequest {
        new: TrendingPostKey {
            post_id: "1".to_string(),
            trending_score: 100,
            created_ts: 1,
            updated_ts: 3,
            club_id: Some("club_1".to_string()),
        },
        nft_canister_ids: vec!["canister_1".to_string()],
    });
    let street_response_after = get_trending_street_posts(GetTrendingStreetPostRequest {
        limit: None,
        cursor: Cursor(None),
    });
    let collection_response_after =
        get_trending_collection_posts(GetTrendingCollectionPostRequest {
            canister_id: "canister_1".to_string(),
            limit: None,
            cursor: Cursor(None),
        });

    // Assert
    // First post should be more trending after update
    assert_eq!(
        street_response_before.posts[0]
            .club_post
            .clone()
            .unwrap()
            .post_id,
        "2"
    );
    assert_eq!(
        street_response_after.posts[0]
            .club_post
            .clone()
            .unwrap()
            .post_id,
        "1"
    );
    assert_eq!(
        collection_response_before.posts[0]
            .club_post
            .clone()
            .unwrap()
            .post_id,
        "2"
    );
    assert_eq!(
        collection_response_after.posts[0]
            .club_post
            .clone()
            .unwrap()
            .post_id,
        "1"
    );
}

/**
1. 2 reply and 1 emoji to first post
2. 1 reply and 1 emoji to second post
3. 1 emoji to third post
*/
fn make_posts_trending(first: &String, second: &String, third: &String) {
    // 2 reply and 1 emoji to first post
    reply_post(ReplyPostRequest {
        reply_id: "1".to_string(),
        user: "peter".to_string(),
        post_id: first.clone(),
        nfts: vec![],
        created_ts: 1,
        words: "this is 1st reply".to_string(),
    });
    reply_post(ReplyPostRequest {
        reply_id: "2".to_string(),
        user: "peter".to_string(),
        post_id: first.clone(),
        nfts: vec![],
        created_ts: 2,
        words: "this is 2nd reply".to_string(),
    });
    react_emoji(ReactEmojiRequest {
        post_id: Some(first.clone()),
        reply_id: None,
        emoji: "👍".to_string(),
        user: "ryan".to_string(),
        created_ts: 2,
    });

    // 1 reply and 1 emoji to second post
    reply_post(ReplyPostRequest {
        reply_id: "3".to_string(),
        user: "peter".to_string(),
        post_id: second.clone(),
        nfts: vec![],
        created_ts: 2,
        words: "this is 1st reply".to_string(),
    });
    react_emoji(ReactEmojiRequest {
        post_id: Some(second.clone()),
        reply_id: None,
        emoji: "👍".to_string(),
        user: "ryan".to_string(),
        created_ts: 2,
    });

    // 1 emoji to third post
    react_emoji(ReactEmojiRequest {
        post_id: Some(third.clone()),
        reply_id: None,
        emoji: "👍".to_string(),
        user: "ryan".to_string(),
        created_ts: 2,
    });
}

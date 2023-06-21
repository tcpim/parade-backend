use super::helpers::*;
use crate::api::post_api::*;
use crate::api::post_user_api::get_posts_by_user;
use crate::api_interface::common_interface::Cursor;
use crate::api_interface::posts_interface::*;
use crate::models::nft_model::NftToken;
use crate::models::post_collection_model::CollectionPostCreatedTsKey;
use crate::models::post_street_model::PostCreatedTsKey;
use crate::models::post_user_model::UserPostCreatedTsKey;

#[test]
fn create_and_get_posts_with_pagination() {
    // set up
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), vec![]);
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "peter".to_string(), vec![]);
    let create_post_request_3 =
        generate_create_post_request(2, "hi_3".to_string(), "ryan".to_string(), vec![]);

    // act
    create_street_post(create_post_request_1);
    create_street_post(create_post_request_2);
    create_street_post(create_post_request_3);

    let request = GetStreetPostsRequest {
        limit: Option::Some(2),
        cursor: Cursor(None),
    };
    let response = get_street_posts(request);

    // assert
    // Get first page
    assert_eq!(response.posts.len(), 2);
    assert_eq!(
        response.posts.get(0).unwrap().post.clone().unwrap().words,
        "hi_3"
    );
    assert_eq!(
        response.posts.get(1).unwrap().post.clone().unwrap().words,
        "hi_2"
    );
    assert_eq!(
        response.next_cursor,
        Cursor(Some(PostCreatedTsKey {
            created_ts: 0,
            post_id: "0".to_string(),
            club_id: None,
        }))
    );

    // Get second page which is also the last page
    let request = GetStreetPostsRequest {
        limit: Option::Some(2),
        cursor: response.next_cursor,
    };
    let response = get_street_posts(request);

    assert_eq!(response.posts.len(), 1);
    assert_eq!(
        response.posts.get(0).unwrap().post.clone().unwrap().words,
        "hi_1"
    );
    assert_eq!(response.next_cursor, Cursor(None));
}

#[test]
fn create_and_get_posts_by_user_pagination() {
    // Set up
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), vec![]);
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "tim".to_string(), vec![]);
    let create_post_request_3 =
        generate_create_post_request(2, "hi_3".to_string(), "zz".to_string(), vec![]);

    // Act
    create_street_post(create_post_request_1);
    create_street_post(create_post_request_2);
    create_street_post(create_post_request_3);
    let response = get_posts_by_user(GetUserPostsRequest {
        user_id: "tim".to_string(),
        limit: Some(1),
        cursor: Cursor(None),
    });

    // Assert
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.posts[0].post.clone().unwrap().words, "hi_2");
    assert_eq!(
        response.next_cursor,
        Cursor(Some(UserPostCreatedTsKey {
            created_ts: 0,
            user_id: "tim".to_string(),
            post_id: "0".to_string(),
            club_id: None,
        }))
    );

    // get second page
    let response = get_posts_by_user(GetUserPostsRequest {
        user_id: "tim".to_string(),
        limit: Some(2),
        cursor: response.next_cursor,
    });
    assert_eq!(response.posts.len(), 1);
    assert_eq!(
        response.posts.get(0).unwrap().post.clone().unwrap().words,
        "hi_1"
    );
    assert_eq!(response.next_cursor, Cursor(None));
}

#[test]
fn create_and_get_posts_by_collection_pagination() {
    // set up
    let nft_1 = vec![NftToken {
        canister_id: "canister_1".to_string(),
        token_index: 0,
        token_id: "token_1".to_string(),
        collection_name: "collection_1".to_string(),
        original_image_url: "".to_string(),
        original_thumbnail_url: "".to_string(),
    }];
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), nft_1.clone());
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "tim".to_string(), nft_1.clone());

    // act
    create_street_post(create_post_request_1);
    create_street_post(create_post_request_2);
    let response = get_posts_by_collection(GetCollectionPostsRequest {
        canister_id: "canister_1".to_string(),
        limit: Option::Some(1),
        cursor: Cursor(None),
    });

    // assert
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.posts[0].post.clone().unwrap().words, "hi_2");
    assert_eq!(
        response.next_cursor,
        Cursor(Some(CollectionPostCreatedTsKey {
            created_ts: 0,
            canister_id: "canister_1".to_string(),
            post_id: "0".to_string(),
            club_id: None,
        }))
    );

    // get second page
    let response = get_posts_by_collection(GetCollectionPostsRequest {
        canister_id: "canister_1".to_string(),
        limit: Option::Some(2),
        cursor: response.next_cursor,
    });
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.next_cursor, Cursor(None));
    assert_eq!(response.posts[0].post.clone().unwrap().words, "hi_1");
}

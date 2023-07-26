use super::helpers::*;
use crate::api::post_api::*;
use crate::api_interface::common_interface::Cursor;
use crate::api_interface::posts_interface::*;
use crate::models::nft_model::NftToken;
use crate::models::post_collection_model::CollectionPostCreatedTsKey;
use crate::models::post_model::PostCreatedTsKey;
use async_std::task;

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
    task::block_on(create_post(create_post_request_1));
    task::block_on(create_post(create_post_request_2));
    task::block_on(create_post(create_post_request_3));

    let request = GetPostsRequest {
        limit: Some(2),
        cursor: Cursor(None),
    };
    let response = get_posts(request);

    // assert
    // Get first page
    assert_eq!(response.posts.len(), 2);
    assert_eq!(response.posts.get(0).unwrap().words, "hi_3");
    assert_eq!(response.posts.get(1).unwrap().words, "hi_2");
    assert_eq!(
        response.next_cursor,
        Cursor(Some(PostCreatedTsKey {
            created_ts: 0,
            post_id: "0".to_string()
        }))
    );

    // Get second page which is also the last page
    let request = GetPostsRequest {
        limit: Some(2),
        cursor: response.next_cursor,
    };
    let response = get_posts(request);

    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.posts.get(0).unwrap().words, "hi_1");
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

        image_url: "".to_string(),
        image_onchain_url: "".to_string(),
        image_height_width_ratio: "".to_string(),
        image_type: "".to_string(),
        image_thumbnail_url: "".to_string(),
        club_id: "".to_string(),
    }];
    let create_post_request_1 =
        generate_create_post_request(0, "hi_1".to_string(), "tim".to_string(), nft_1.clone());
    let create_post_request_2 =
        generate_create_post_request(1, "hi_2".to_string(), "tim".to_string(), nft_1);

    // act
    task::block_on(create_post(create_post_request_1));
    task::block_on(create_post(create_post_request_2));
    let response = get_posts_by_collection(GetCollectionPostsRequest {
        canister_id: "canister_1".to_string(),
        limit: Some(1),
        cursor: Cursor(None),
    });

    // assert
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.posts[0].words, "hi_2");
    assert_eq!(
        response.next_cursor,
        Cursor(Some(CollectionPostCreatedTsKey {
            created_ts: 0,
            canister_id: "canister_1".to_string(),
            post_id: "0".to_string()
        }))
    );

    // get second page
    let response = get_posts_by_collection(GetCollectionPostsRequest {
        canister_id: "canister_1".to_string(),
        limit: Some(2),
        cursor: response.next_cursor,
    });
    assert_eq!(response.posts.len(), 1);
    assert_eq!(response.next_cursor, Cursor(None));
    assert_eq!(response.posts[0].words, "hi_1");
}

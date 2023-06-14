use super::helpers::*;
use crate::api::post::*;
use crate::api::post_deletion::*;
use crate::api_interface::common::Cursor;
use crate::api_interface::posts::*;
use crate::models::nft::NftToken;

#[test]
fn create_and_delete_posts() {
    let nft_1 = NftToken {
        canister_id: "canister_1".to_string(),
        token_index: 0,
        token_id: "token_1".to_string(),
        collection_name: "collection_1".to_string(),
        original_image_url: "".to_string(),
        original_thumbnail_url: "".to_string(),
    };
    let create_post_request_1 = generate_create_post_request(
        0,
        "hi_1".to_string(),
        "tim".to_string(),
        vec![nft_1.clone()],
    );
    let create_post_request_2 = generate_create_post_request(
        1,
        "hi_2".to_string(),
        "peter".to_string(),
        vec![nft_1.clone()],
    );
    let create_post_request_3 = generate_create_post_request(
        2,
        "hi_3".to_string(),
        "ryan".to_string(),
        vec![nft_1.clone()],
    );

    let post_1 = create_post(create_post_request_1);
    create_post(create_post_request_2);
    create_post(create_post_request_3);

    delete_post(post_1.post.id.0.clone());

    let get_street_posts_request = GetPostsRequest {
        limit: None,
        cursor: Cursor(None),
    };
    let response = get_posts(get_street_posts_request);
    assert_eq!(response.posts.len(), 2);
    assert_eq!(response.next_cursor, Cursor(None));

    delete_all_post();

    let request = GetPostsRequest {
        limit: None,
        cursor: Cursor(None),
    };
    let response = get_posts(request);
    assert_eq!(response.posts.len(), 0);
    assert_eq!(response.next_cursor, Cursor(None));

    let request = GetCollectionPostsRequest {
        canister_id: "canister_1".to_string(),
        limit: None,
        cursor: Cursor(None),
    };
    let response = get_posts_by_collection(request);
    assert_eq!(response.posts.len(), 0);
    assert_eq!(response.next_cursor, Cursor(None));
}

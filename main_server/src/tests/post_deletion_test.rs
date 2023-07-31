use super::helpers::*;
use crate::api::post_api::*;
use crate::api::post_deletion_api::*;
use crate::api_interface::common_interface::Cursor;
use crate::api_interface::posts_interface::*;
use crate::models::nft_model::NftToken;

#[test]
fn create_and_delete_posts() {
    let nft_1 = NftToken {
        canister_id: "canister_1".to_string(),
        token_index: 0,
        token_id: "token_1".to_string(),
        collection_name: "collection_1".to_string(),
        image_url: "".to_string(),
        image_thumbnail_url: "".to_string(),
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

    let post_1 = create_street_post(create_post_request_1);
    create_street_post(create_post_request_2);
    create_street_post(create_post_request_3);

    delete_post(post_1.post.id.0.clone());

    let get_street_posts_request = GetStreetPostsRequest {
        limit: None,
        cursor: Cursor(None),
    };
    let get_street_posts_response = get_street_posts(get_street_posts_request);
    assert_eq!(get_street_posts_response.posts.len(), 2);
    assert_eq!(get_street_posts_response.next_cursor, Cursor(None));

    dlp();

    let request = GetStreetPostsRequest {
        limit: None,
        cursor: Cursor(None),
    };
    let response = get_street_posts(request);
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

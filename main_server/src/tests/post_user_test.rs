use crate::api::inter_canister_apis::add_club_post_to_user;
use crate::api::post_user_api::get_posts_by_user;
use crate::api_interface::common_interface::Cursor;
use crate::api_interface::inter_canister_interface::AddClubPostToUserRequest;
use crate::api_interface::posts_interface::*;
use crate::models::post_user_model::UserPostCreatedTsKey;

#[test]
fn add_post_to_user_test() {
    // set up
    let request_1 = AddClubPostToUserRequest {
        caller: "caller".to_string(),
        user_post_created_key: UserPostCreatedTsKey {
            user_id: "tim".to_string(),
            created_ts: 0,
            post_id: "1".to_string(),
            club_id: Some("club_1".to_string()),
        },
    };
    let request_2 = AddClubPostToUserRequest {
        caller: "caller".to_string(),
        user_post_created_key: UserPostCreatedTsKey {
            user_id: "tim".to_string(),
            created_ts: 1,
            post_id: "2".to_string(),
            club_id: Some("club_1".to_string()),
        },
    };
    add_club_post_to_user(request_1);
    add_club_post_to_user(request_2);

    // act
    let request = GetUserPostsRequest {
        user_id: "tim".to_string(),
        limit: None,
        cursor: Cursor(None),
    };
    let response = get_posts_by_user(request);

    // assert
    assert_eq!(response.posts.len(), 2);
    assert_eq!(response.next_cursor, Cursor(None));
    assert_eq!(response.posts[0].club_post.clone().unwrap().post_id, "2");
    assert_eq!(response.posts[1].club_post.clone().unwrap().post_id, "1");
}

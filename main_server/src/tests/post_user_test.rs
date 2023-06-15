use crate::api::post_user::{add_club_post_to_user, get_posts_by_user};
use crate::api_interface::common::Cursor;
use crate::api_interface::posts::*;
use crate::models::post_user::UserPostCreatedTsKey;

#[test]
fn add_post_to_user_test() {
    // set up
    let request_1 = UserPostCreatedTsKey {
        user_id: "tim".to_string(),
        created_ts: 0,
        post_id: "1".to_string(),
        club_id: Some("club_1".to_string()),
    };
    let request_2 = UserPostCreatedTsKey {
        user_id: "tim".to_string(),
        created_ts: 1,
        post_id: "2".to_string(),
        club_id: Some("club_1".to_string()),
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

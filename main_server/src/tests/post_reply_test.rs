use crate::api::post_api::*;
use crate::api::post_reply_api::*;
use crate::api_interface::post_reply_interface::*;

#[test]
pub fn reply_post_test() {
    let create_post_request = super::helpers::generate_create_post_request(
        0,
        "hi_1".to_string(),
        "tim".to_string(),
        vec![],
    );
    let create_post_res = create_street_post(create_post_request);
    let post_id_str = create_post_res.post.id.0.clone();
    let reply_post_request_1 = ReplyPostRequest {
        reply_id: "1".to_string(),
        user: "peter".to_string(),
        post_id: post_id_str.clone(),
        nfts: vec![],
        created_ts: 1,
        words: "this is 1st reply".to_string(),
    };
    let reply_post_request_2 = ReplyPostRequest {
        reply_id: "2".to_string(),
        user: "ryan".to_string(),
        post_id: post_id_str.clone(),
        nfts: vec![],
        created_ts: 2,
        words: "this is 2nd reply".to_string(),
    };
    let reply_post_request_3 = ReplyPostRequest {
        reply_id: "3".to_string(),
        user: "alex".to_string(),
        post_id: post_id_str.clone(),
        nfts: vec![],
        created_ts: 3,
        words: "this is 3rd reply".to_string(),
    };
    reply_post(reply_post_request_1);
    reply_post(reply_post_request_2);
    reply_post(reply_post_request_3);

    let get_posts_res = get_street_post_by_id(post_id_str.clone());

    assert_eq!(get_posts_res.post.as_ref().unwrap().replies.len(), 3);

    // Get first page
    let get_replies_res = get_post_replies(GetPostRepliesRequest {
        post_id: post_id_str.clone(),
        offset: 0,
        limit: Some(2),
    });
    assert_eq!(get_replies_res.post_replies.len(), 2);
    assert_eq!(get_replies_res.offset, 2);
    assert_eq!(get_replies_res.post_replies[0].words, "this is 1st reply");
    assert_eq!(get_replies_res.post_replies[1].words, "this is 2nd reply");

    // Get last page
    let get_replies_res = get_post_replies(GetPostRepliesRequest {
        post_id: post_id_str.clone(),
        offset: get_replies_res.offset,
        limit: Some(2),
    });
    assert_eq!(get_replies_res.post_replies.len(), 1);
    assert_eq!(get_replies_res.offset, 3);
    assert_eq!(get_replies_res.post_replies[0].words, "this is 3rd reply");
}

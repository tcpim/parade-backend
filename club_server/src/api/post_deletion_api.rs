use crate::stable_structure::access_helper::*;
use candid::candid_method;
use ic_cdk_macros::update;

use crate::api_interface::common_interface::*;
use crate::api_interface::posts_interface::*;
use crate::models::post_model::*;

#[update]
#[candid_method(update)]
pub fn delete_post(post_id: String) -> DeletePostResponse {
    with_post_by_id_mut(
        |post_by_id| match post_by_id.remove(&PostIdString(post_id.clone())) {
            Some(_) => DeletePostResponse { error: None },
            None => DeletePostResponse {
                error: Some(ServerError::DeletePostError(format!(
                    "Failed to delete post by id: {}",
                    post_id
                ))),
            },
        },
    )
}

#[update]
#[candid_method(update)]
pub fn delete_all_post() {
    // You might run into candid deserialization error if you changed the proto
    // Solution: you can temporarily modify the proto to fit the earlier version and make the deserialization work
    // and then change it back after deletion
    with_post_by_id_mut(|storage| {
        let posts: Vec<_> = storage.range(..).collect();
        for (key, _value) in posts {
            storage.remove(&key);
        }
    });

    with_club_posts_created_mut(|storage| {
        let posts: Vec<_> = storage.range(..).collect();
        for (key, _value) in posts {
            storage.remove(&key);
        }
    });

    with_trending_posts_mut(|max_heap| {
        let posts: Vec<_> = max_heap.range(..).collect();
        for (key, _value) in posts {
            max_heap.remove(&key);
        }
    });

    with_collection_posts_created_mut(|storage| {
        let posts: Vec<_> = storage.range(..).collect();
        for (key, _value) in posts {
            storage.remove(&key);
        }
    });

    with_trending_posts_collection_mut(|storage| {
        let posts: Vec<_> = storage.range(..).collect();
        for (key, _value) in posts {
            storage.remove(&key);
        }
    });
}

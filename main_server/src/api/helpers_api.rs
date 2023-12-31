use crate::api::constants::DEFAULT_PAGE_SIZE;
use crate::api_interface::common_interface::*;
use crate::api_interface::posts_interface::PostType;
use crate::models::post_club_model::{ClubPost, HasClubId};
use crate::models::post_model::*;
use crate::models::trending_post_collection_model::TrendingPostCollectionKey;
use crate::models::trending_post_model::TrendingPostKey;
use crate::stable_structure::access_helper::*;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap};

use super::constants::FRONTEND_CANISTER_ID_PROD;

/**
Given a btree and a start key, return a page of posts with max len = limit and the next cursor
*/
pub fn get_page_from_btree<T: HasPostId + HasClubId + BoundedStorable + Ord + Clone>(
    btree: &StableBTreeMap<T, (), VirtualMemory<DefaultMemoryImpl>>,
    start: T,
    end: T,
    limit: usize,
) -> (Vec<PostType>, Cursor<T>) {
    let mut res = vec![];
    let mut iter = btree.range(start..=end).map(|x| x.0).peekable();
    for _ in 0..limit {
        if let Some(item) = iter.next() {
            if item.club_id().is_some() {
                res.push(PostType {
                    post: None,
                    club_post: Some(ClubPost {
                        post_id: item.post_id().clone(),
                        club_id: item.club_id().clone().unwrap(),
                    }),
                })
            } else {
                match get_post_by_id_from_store(&PostIdString(item.post_id().clone())) {
                    Some(post) => res.push(PostType {
                        post: Some(post),
                        club_post: None,
                    }),
                    None => println!(
                        "Didn't find post id in post_by_id: {:?}",
                        item.post_id().clone()
                    ),
                }
            }
        } else {
            // Not enough items for one page. The next cursor will be None
            break;
        }
    }

    let next = iter.peek().cloned();
    (res, Cursor(next))
}

/**
Get a post from key value store by post id
 */
pub fn get_post_by_id_from_store(post_id: &PostIdString) -> Option<Post> {
    with_post_by_id(|post_by_id| post_by_id.get(post_id))
}

pub fn add_post_reply_to_reply_store(post_reply: PostReply) -> Option<ServerError> {
    with_post_reply_by_id_mut(|post_reply_by_id| {
        let reply_opt = post_reply_by_id.get(&post_reply.id);
        if reply_opt.is_some() {
            return Some(ServerError {
                api_name: "reply_post".to_string(),
                error_message: format!("Post reply id already exists: {:?}", &post_reply.id),
            });
        }
        post_reply_by_id.insert(post_reply.id.clone(), post_reply);
        None
    })
}

/**
Get a page of post replies given offset and limit, return the next offset
*/
pub fn get_post_replies_from_reply_ids(
    reply_ids: Vec<PostReplyIdString>,
    offset: i32,
    limit: Option<i32>,
) -> (Vec<PostReply>, i32) {
    let mut result_vec: Vec<PostReply> = vec![];

    // Get one page of replies from offset to min(offset + limit-1, len - 1)
    if reply_ids.is_empty() || offset < 0 || offset >= reply_ids.len() as i32 {
        return (result_vec, reply_ids.len() as i32);
    }
    let end = std::cmp::min(
        offset + limit.unwrap_or(DEFAULT_PAGE_SIZE) - 1,
        reply_ids.len() as i32 - 1,
    );

    with_post_reply_by_id(|post_reply_by_id| {
        for i in offset..=end {
            match post_reply_by_id.get(&reply_ids[i as usize]) {
                Some(post_reply) => result_vec.push(post_reply.clone()),
                None => {
                    println!(
                        "Failed to find post reply in POST_REPLY_BY_ID with post_reply_id: {:?}",
                        &reply_ids[i as usize].0
                    );
                }
            }
        }
    });

    (result_vec, end + 1) // next offset is end + 1
}

// Construct a new TrendingPostKey with score = num_replies * 1 + num_emojis * 1
pub fn get_trending_post_key(post: &Post, club_id: Option<String>) -> TrendingPostKey {
    let num_reactions = post.emoji_reactions.iter().fold(0, |acc, (_, v)| acc + v);
    let trending_score = post.replies.len() as u32 + num_reactions;
    TrendingPostKey {
        trending_score,
        post_id: post.id.0.clone(),
        created_ts: post.created_ts,
        updated_ts: post.created_ts,
        club_id,
    }
}

/**
Update trending score in trending indexes
Trending street, trending collection posts
*/
pub fn update_trending_post_indexes(new_post: &Post, club_id: Option<String>) {
    let new_key = get_trending_post_key(new_post, club_id);

    // update trending score in street trending
    with_trending_posts_street_mut(|storage| {
        // TODO: the new key's postId and clubId should uniquely identify a post, but the insert should overwrite it
        storage.remove(&new_key);
        storage.insert(new_key.clone(), ());
    });

    // update trending score in collection trending
    if !new_post.nfts.is_empty() {
        // Currently only support one collection per post
        let canister_id = new_post.nfts[0].canister_id.clone();

        with_trending_posts_collection_mut(|max_heap| {
            // TODO: the new key's postId and clubId should uniquely identify a post, but the insert should overwrite it
            max_heap.remove(&TrendingPostCollectionKey {
                canister_id: canister_id.clone(),
                trending_info: new_key.clone(),
            });
            max_heap.insert(
                TrendingPostCollectionKey {
                    canister_id: canister_id.clone(),
                    trending_info: new_key.clone(),
                },
                (),
            );
        })
    }
}

/**
Update trending score in trending indexes for club post
*/
pub fn update_trending_club_post_indexes(new_key: &TrendingPostKey, nft_canister_ids: Vec<String>) {
    // update trending score in street trending
    with_trending_posts_street_mut(|storage| {
        // TODO: the new key's postId and clubId should uniquely identify a post, but the insert should overwrite it
        storage.remove(new_key);
        storage.insert(new_key.clone(), ());
    });

    // update trending score in collection trending
    if !nft_canister_ids.is_empty() {
        // Currently only support one collection per post
        let canister_id = nft_canister_ids[0].clone();

        with_trending_posts_collection_mut(|max_heap| {
            max_heap.remove(&TrendingPostCollectionKey {
                canister_id: canister_id.clone(),
                trending_info: new_key.clone(),
            });
            max_heap.insert(
                TrendingPostCollectionKey {
                    canister_id: canister_id.clone(),
                    trending_info: new_key.clone(),
                },
                (),
            );
        })
    }
}

// If the caller is anonymous, return false
pub fn is_caller_authorized() -> bool {
    if is_run_in_dev() || is_run_in_unit_test() {
        return true;
    }

    let caller = ic_cdk::api::caller().to_string();
    if caller == "" || caller == "2vxsx-fae" {
        return false;
    }

    true
    // TODO: this doesn;t work. See https://forum.dfinity.org/t/only-allow-update-call-from-frontend-canister/21936
    // if is_run_in_prod() {
    //     let caller = ic_cdk::api::caller().to_string();
    //     if caller.eq(FRONTEND_CANISTER_ID_PROD) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
}

// Reason for this is because the inter canister call destination canister cannot use ic_cdk::api::caller()
// See https://forum.dfinity.org/t/canister-violated-contract-ic0-msg-caller-size-cannot-be-executed-in-reply-callback-mode/7890
pub fn is_inter_canister_caller_authorized(caller: String) -> bool {
    if is_run_in_dev() || is_run_in_unit_test() {
        return true;
    }

    // TODO: this doesn;t work. See https://forum.dfinity.org/t/only-allow-update-call-from-frontend-canister/21936
    // if caller.eq(FRONTEND_CANISTER_ID_PROD) {
    //     return true;
    // }

    // return false;

    return true;
}

fn is_run_in_unit_test() -> bool {
    with_canister_args(|cell| {
        let canister_args = cell.get();
        canister_args.env.eq("")
    })
}

fn is_run_in_dev() -> bool {
    with_canister_args(|cell| {
        let canister_args = cell.get();
        canister_args.env.eq("dev")
    })
}

fn is_run_in_prod() -> bool {
    with_canister_args(|cell| {
        let canister_args = cell.get();
        canister_args.env.eq("prod")
    })
}

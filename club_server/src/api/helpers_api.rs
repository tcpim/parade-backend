use crate::api::constants::DEFAULT_PAGE_SIZE;
use crate::api_interface::common_interface::*;
use crate::models::nft_model::NftToken;
use crate::models::post_model::*;
use crate::models::trending_post_collection_model::TrendingPostCollectionKey;
use crate::models::trending_post_model::TrendingPostKey;
use crate::stable_structure::access_helper::*;
use candid::{CandidType, Principal};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap};
use std::panic;

/**
Given a btree and a start key, return a page of posts with max len = limit and the next cursor
*/
pub fn get_page_from_btree<T: HasPostId + BoundedStorable + Ord + Clone>(
    btree: &StableBTreeMap<T, (), VirtualMemory<DefaultMemoryImpl>>,
    start: T,
    end: T,
    limit: usize,
) -> (Vec<Post>, Cursor<T>) {
    let mut res = vec![];
    let mut iter = btree.range(start..=end).map(|x| x.0).peekable();
    for _ in 0..limit {
        if let Some(item) = iter.next() {
            match get_post_by_id_from_store(&PostIdString(item.post_id().clone())) {
                Some(post) => res.push(post.clone()),
                None => println!(
                    "Didn't find post id in post_by_id: {:?}",
                    item.post_id().clone()
                ),
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
                error_message: format!(
                    "Should not happen due to uuid! Post reply id already exists: {:?}",
                    &post_reply.id
                ),
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

/**
Construct a new TrendingPostKey with score = num_replies * 1 + num_emojis * 1
**/
pub fn get_trending_post_key(post: &Post) -> TrendingPostKey {
    let num_reactions = post.emoji_reactions.iter().fold(0, |acc, (_, v)| acc + v);
    let trending_score = post.replies.len() as u32 + num_reactions;
    TrendingPostKey {
        trending_score,
        post_id: post.id.0.clone(),
        created_ts: post.created_ts,
        updated_ts: post.updated_ts,
    }
}

/**
Update trending score in trending indexes
trending collection posts, trending club posts
*/
pub fn update_trending_post_indexes(old_post: Post, new_trending_score: &TrendingPostKey) {
    // update trending score in trending
    with_trending_posts_mut(|max_heap| {
        max_heap.remove(new_trending_score);
        max_heap.insert(new_trending_score.clone(), ());
    });

    // update trending score in collection trending
    if !old_post.nfts.is_empty() {
        // Currently only support one collection per post
        let canister_id = old_post.nfts[0].canister_id.clone();

        with_trending_posts_collection_mut(|max_heap| {
            max_heap.remove(&TrendingPostCollectionKey {
                canister_id: canister_id.clone(),
                trending_info: new_trending_score.clone(),
            });
            max_heap.insert(
                TrendingPostCollectionKey {
                    canister_id: canister_id.clone(),
                    trending_info: new_trending_score.clone(),
                },
                (),
            );
        })
    }
}

pub fn convert_to_main_server_nfttoken(
    a: Vec<NftToken>,
) -> Vec<crate::api_interface::inter_canister::NftTokenExternal> {
    a.into_iter()
        .map(|x| crate::api_interface::inter_canister::NftTokenExternal {
            canister_id: x.canister_id,
            token_index: x.token_index,
            token_id: x.token_id,
            collection_name: x.collection_name,
            original_image_url: x.original_image_url,
            original_thumbnail_url: x.original_thumbnail_url,
        })
        .collect()
}

pub fn get_club_id() -> String {
    with_club_info(|cell| {
        let club_info = cell.get();
        club_info.club_id.clone()
    })
}

pub async fn call_inter_canister_async<T: CandidType>(
    canister_id: &str,
    method_name: &str,
    request: T,
    err_msg: &str,
) {
    println!("{} is called expectedly!", method_name);

    if is_within_canister() {
        // If within canister, call directly
        ic_cdk::api::call::call::<(T,), ()>(
            Principal::from_text(canister_id).unwrap(),
            method_name,
            (request,),
        )
        .await
        .expect(err_msg);
    }
}

fn is_within_canister() -> bool {
    let result = panic::catch_unwind(|| {
        // If panic, then it is run by unit test (not within canister)
        println!("Current canister ID is : {}", ic_cdk::api::id());
    });

    result.is_ok()
}

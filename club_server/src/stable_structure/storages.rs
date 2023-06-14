use crate::models::post::{Post, PostCreatedTsKey, PostIdString, PostReply, PostReplyIdString};
use crate::models::post_collection::CollectionPostCreatedTsKey;
use crate::models::trending_post::TrendingPostKey;
use crate::models::trending_post_collection::TrendingPostCollectionKey;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type PostByIdMap = StableBTreeMap<PostIdString, Post, Memory>;
pub type PostReplyByIdMap = StableBTreeMap<PostReplyIdString, PostReply, Memory>;
pub type PostsCreatedHeap = StableBTreeMap<PostCreatedTsKey, (), Memory>;
pub type CollectionPostsCreatedHeap = StableBTreeMap<CollectionPostCreatedTsKey, (), Memory>;
pub type TrendingPostClubHeap = StableBTreeMap<TrendingPostKey, (), Memory>;
pub type TrendingPostCollectionHeap = StableBTreeMap<TrendingPostCollectionKey, (), Memory>;

pub const POST_BY_ID_MEMORY_ID: MemoryId = MemoryId::new(0);
pub const POST_REPLIES_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const POSTS_CREATED_MEMORY_ID: MemoryId = MemoryId::new(2);
pub const COLLECTION_POSTS_CREATED_MEMORY_ID: MemoryId = MemoryId::new(3);
pub const TRENDING_POST_MEMORY_ID: MemoryId = MemoryId::new(4);
pub const TRENDING_POST_COLLECTION_MEMORY_ID: MemoryId = MemoryId::new(5);

thread_local! {
// initiate a memory manager
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static POST_BY_ID: RefCell<StableBTreeMap<PostIdString, Post, Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(POST_BY_ID_MEMORY_ID)
                )
            )
        );

    pub static POST_REPLY_BY_ID: RefCell<StableBTreeMap<PostReplyIdString, PostReply, Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(POST_REPLIES_MEMORY_ID)
                )
            )
        );

    // Database
    // Usually store one to many relation in a BTreeMap with composite key

    // Store all posts for this club
    pub static CLUB_POSTS_CREATED: RefCell<StableBTreeMap<PostCreatedTsKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
             RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(POSTS_CREATED_MEMORY_ID)
                )
            )
        );

    // Store all posts for each collection
    pub static COLLECTION_POSTS_CREATED: RefCell<StableBTreeMap<CollectionPostCreatedTsKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager| {
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(COLLECTION_POSTS_CREATED_MEMORY_ID)
                )
            )
        });

    // Secondary indexes for posts

    // Store all trending posts for each club
    pub static TRENDING_POSTS: RefCell<StableBTreeMap<TrendingPostKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(TRENDING_POST_MEMORY_ID)
                )
            )
        );

    // Store all trending posts for each collection
    pub static TRENDING_POSTS_COLLECTION: RefCell<StableBTreeMap<TrendingPostCollectionKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(TRENDING_POST_COLLECTION_MEMORY_ID)
                )
            )
        );
}

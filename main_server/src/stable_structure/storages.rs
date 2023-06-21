use crate::models::post_collection_model::CollectionPostCreatedTsKey;
use crate::models::post_model::{Post, PostIdString, PostReply, PostReplyIdString};
use crate::models::post_street_model::PostCreatedTsKey;
use crate::models::post_user_model::UserPostCreatedTsKey;
use crate::models::trending_post_collection_model::TrendingPostCollectionKey;
use crate::models::trending_post_model::TrendingPostKey;
use crate::models::user_model::{User, UserNameStringKey, UserPrincipalStringKey};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type UserByIdMap = StableBTreeMap<UserPrincipalStringKey, User, Memory>;
pub type PostByIdMap = StableBTreeMap<PostIdString, Post, Memory>;
pub type PostReplyByIdMap = StableBTreeMap<PostReplyIdString, PostReply, Memory>;
pub type StreetPostsCreatedHeap = StableBTreeMap<PostCreatedTsKey, (), Memory>;
pub type UserPostsCreatedHeap = StableBTreeMap<UserPostCreatedTsKey, (), Memory>;
pub type CollectionPostsCreatedHeap = StableBTreeMap<CollectionPostCreatedTsKey, (), Memory>;
pub type TrendingPostStreetHeap = StableBTreeMap<TrendingPostKey, (), Memory>;
pub type TrendingPostCollectionHeap = StableBTreeMap<TrendingPostCollectionKey, (), Memory>;
pub type UserNamesMap = StableBTreeMap<UserNameStringKey, (), Memory>;

pub const USER_BY_ID_MEMORY_ID: MemoryId = MemoryId::new(0);
pub const POST_BY_ID_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const STREET_POSTS_CREATED_MEMORY_ID: MemoryId = MemoryId::new(2);
pub const USER_POSTS_CREATED_MEMORY_ID: MemoryId = MemoryId::new(3);
pub const POST_REPLIES_MEMORY_ID: MemoryId = MemoryId::new(4);
pub const COLLECTION_POSTS_CREATED_MEMORY_ID: MemoryId = MemoryId::new(5);
pub const TRENDING_POST_STREET_MEMORY_ID: MemoryId = MemoryId::new(6);
pub const TRENDING_POST_COLLECTION_MEMORY_ID: MemoryId = MemoryId::new(7);
pub const USER_NAMES_MEMORY_ID: MemoryId = MemoryId::new(8);

thread_local! {
// initiate a memory manager
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    /**
    One to One relation between key and value
    */
    pub static USER_BY_ID: RefCell<StableBTreeMap<UserPrincipalStringKey, User, Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(USER_BY_ID_MEMORY_ID)
                )
            )
        );

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


    /**
    Unique values
    */
    pub static USER_NAMES: RefCell<StableBTreeMap<UserNameStringKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(USER_NAMES_MEMORY_ID)
                )
            )
        );

    /**
    Database
    Usually store one to many relation in a BTreeMap with composite key
    */
    // Street (public) posts by created time
    pub static STREET_POSTS_CREATED: RefCell<StableBTreeMap<PostCreatedTsKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(STREET_POSTS_CREATED_MEMORY_ID)
                )
            )
        );

    // User posts by created time
    pub static USER_POSTS_CREATED: RefCell<StableBTreeMap<UserPostCreatedTsKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
             RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(USER_POSTS_CREATED_MEMORY_ID)
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
    // Store all trending posts in public street
    pub static TRENDING_POSTS_STREET: RefCell<StableBTreeMap<TrendingPostKey, (), Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(TRENDING_POST_STREET_MEMORY_ID)
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

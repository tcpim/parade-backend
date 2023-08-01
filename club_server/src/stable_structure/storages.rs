use crate::models::chat_model::{ChatClubMessage, ChatClubMessageIdString};
use crate::models::club_model::ClubInfo;
use crate::models::init_model::CanisterArgs;
use crate::models::post_collection_model::CollectionPostCreatedTsKey;
use crate::models::post_model::{
    Post, PostCreatedTsKey, PostIdString, PostReply, PostReplyIdString,
};
use crate::models::trending_post_collection_model::TrendingPostCollectionKey;
use crate::models::trending_post_model::TrendingPostKey;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell, StableVec};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type PostByIdMap = StableBTreeMap<PostIdString, Post, Memory>;
pub type PostReplyByIdMap = StableBTreeMap<PostReplyIdString, PostReply, Memory>;
pub type PostsCreatedHeap = StableBTreeMap<PostCreatedTsKey, (), Memory>;
pub type CollectionPostsCreatedHeap = StableBTreeMap<CollectionPostCreatedTsKey, (), Memory>;
pub type TrendingPostClubHeap = StableBTreeMap<TrendingPostKey, (), Memory>;
pub type TrendingPostCollectionHeap = StableBTreeMap<TrendingPostCollectionKey, (), Memory>;
pub type ClubInfoCell = StableCell<ClubInfo, Memory>;
pub type ChatClubMessageById = StableBTreeMap<ChatClubMessageIdString, ChatClubMessage, Memory>;
pub type ChatClubMessagesVec = StableVec<ChatClubMessageIdString, Memory>;
pub type CanisterArgsCell = StableCell<CanisterArgs, Memory>;

pub const POST_BY_ID_MEMORY_ID: MemoryId = MemoryId::new(0);
pub const POST_REPLIES_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const POSTS_CREATED_MEMORY_ID: MemoryId = MemoryId::new(2);
pub const COLLECTION_POSTS_CREATED_MEMORY_ID: MemoryId = MemoryId::new(3);
pub const TRENDING_POST_MEMORY_ID: MemoryId = MemoryId::new(4);
pub const TRENDING_POST_COLLECTION_MEMORY_ID: MemoryId = MemoryId::new(5);
pub const CLUB_INFO_MEMORY_ID: MemoryId = MemoryId::new(6);
pub const CHAT_MESSAGE_BY_ID_MEMORY_ID: MemoryId = MemoryId::new(7);
pub const CHAT_MESSAGES_VEC_MEMORY_ID: MemoryId = MemoryId::new(8);
pub const CANISTER_ARGS_MEMORY_ID: MemoryId = MemoryId::new(9);

thread_local! {
    // initiate a memory manager
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Main storage
    pub static CLUB_INFO: RefCell<StableCell<ClubInfo, Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableCell::init(memory_manager.borrow().get(CLUB_INFO_MEMORY_ID), ClubInfo {
                club_id: "".to_string(),
                club_name: "".to_string(),
                club_description: "".to_string(),
            }).expect("Failed to init CLUB_INFO")
            )
        );

    pub static CANISTER_ARGS: RefCell<StableCell<CanisterArgs, Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableCell::init(memory_manager.borrow().get(CANISTER_ARGS_MEMORY_ID), CanisterArgs {
                env: "".to_string(),
            }).expect("Failed to init CANISTER_ARGS")
            )
        );

    /**
    Key Value store. Usually key is ID
    */
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

    pub static CHAT_CLUB_MESSAGE_BY_ID: RefCell<StableBTreeMap<ChatClubMessageIdString, ChatClubMessage, Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableBTreeMap::init(
                    memory_manager.borrow().get(CHAT_MESSAGE_BY_ID_MEMORY_ID)
                )
            )
        );


    /**
    Indexes
    Usually store one to many relation in a BTreeMap with composite key
    */
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

    pub static CHAT_CLUB_MESSAGES: RefCell<StableVec<ChatClubMessageIdString, Memory>> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableVec::init(
                    memory_manager.borrow().get(CHAT_MESSAGES_VEC_MEMORY_ID)
                ).expect("Failed to init CHAT_MESSAGES")
            )
        );
}

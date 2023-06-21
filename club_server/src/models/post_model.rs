use crate::models::nft_model::NftToken;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq)]
pub struct Post {
    pub id: PostIdString,
    pub club_id: String,
    pub created_by: String, // user principal
    pub nfts: Vec<NftToken>,
    pub in_public: bool, // whether this post is seenable in public street
    pub words: String,
    pub created_ts: u64,
    pub updated_ts: u64,
    pub replies: Vec<PostReplyIdString>, // string is reply id with {timestamp}-{user principal}-reply
    pub emoji_reactions: BTreeMap<String, u32>, // emoji reactions on a post, key is emoji, value is count
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, CandidType, Deserialize, Debug)]
pub struct PostIdString(pub String);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, CandidType, Deserialize, Debug)]
pub struct PostReplyIdString(pub String);

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq)]
pub struct PostReply {
    pub id: PostReplyIdString,
    pub created_by: String,                     // user who replied
    pub post_id: PostIdString,                  // the post being replied. See Post.id
    pub words: String,                          // the reply content
    pub created_ts: u64,                        // the timestamp when this reply is created
    pub nfts: Vec<NftToken>,                    // user can reply with NFTs
    pub emoji_reactions: BTreeMap<String, u32>, // emoji reactions on a reply, key is emoji, value is count
}

pub trait HasPostId {
    fn post_id(&self) -> String;
}

#[derive(PartialEq, Eq, Clone, CandidType, Deserialize, Debug)]
pub struct PostCreatedTsKey {
    pub created_ts: u64,
    pub post_id: String,
}

impl Ord for PostCreatedTsKey {
    // Sort by created ts in descending order
    // Note!!: do reverse compare on created ts, since this is a max heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.post_id == other.post_id {
            return std::cmp::Ordering::Equal;
        }

        other.created_ts.cmp(&self.created_ts)
    }
}

impl PartialOrd for PostCreatedTsKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HasPostId for PostCreatedTsKey {
    fn post_id(&self) -> String {
        self.post_id.clone()
    }
}

// ######################
// Traits implementations for stable structures
// ######################
const MAX_POST_SIZE: u32 = 50000; // 50KB
const MAX_POST_REPLY_SIZE: u32 = 5000; // 2KB
const MAX_POST_ID_STRING_KEY_SIZE: u32 = 50; // 100Bytes, usually uuid
const MAX_POST_CREATED_TS_SIZE: u32 = 100; // 50B for post id and 50B for timestamp

impl Storable for Post {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode Post for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode Post from stable storage")
    }
}

impl BoundedStorable for Post {
    const MAX_SIZE: u32 = MAX_POST_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for PostReply {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode PostReply for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode PostReply from stable storage")
    }
}

impl BoundedStorable for PostReply {
    const MAX_SIZE: u32 = MAX_POST_REPLY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for PostCreatedTsKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode ClubPostCreatedTsKey for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode ClubPostCreatedTsKey from stable storage")
    }
}

impl BoundedStorable for PostCreatedTsKey {
    const MAX_SIZE: u32 = MAX_POST_CREATED_TS_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for PostIdString {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }
}

impl BoundedStorable for PostIdString {
    const MAX_SIZE: u32 = MAX_POST_ID_STRING_KEY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for PostReplyIdString {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }
}

impl BoundedStorable for PostReplyIdString {
    const MAX_SIZE: u32 = MAX_POST_ID_STRING_KEY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

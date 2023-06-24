use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, CandidType, PartialEq, Deserialize)]
pub struct ChatClubMessage {
    pub id: ChatClubMessageIdString, // uuid from frontend
    pub user_id: String,             // user wallet principal id
    pub created_ts: u64,
    pub words: String,
    pub updated_ts: u64,
    pub emoji_reactions: BTreeMap<String, u32>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, CandidType, Deserialize, Debug)]
pub struct ChatClubMessageIdString(pub String);

// ######################
// Traits implementations for stable structures
// ######################
const MAX_CHAT_MESSAGE_SIZE: u32 = 1000; // 1KB
const MAX_CHAT_MESSAGE_ID_STRING_KEY_SIZE: u32 = 100; // 100Bytes, usually uuid

impl BoundedStorable for ChatClubMessage {
    const MAX_SIZE: u32 = MAX_CHAT_MESSAGE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for ChatClubMessage {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode ChatMessage for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to ChatMessage Post from stable storage")
    }
}

impl Storable for ChatClubMessageIdString {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }
}

impl BoundedStorable for ChatClubMessageIdString {
    const MAX_SIZE: u32 = MAX_CHAT_MESSAGE_ID_STRING_KEY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

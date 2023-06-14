use crate::models::post::HasPostId;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;
use crate::models::post_club::HasClubId;

#[derive(PartialEq, Eq, Clone, CandidType, Deserialize, Debug)]
pub struct PostCreatedTsKey {
    pub post_id: String,
    pub club_id: Option<String>,
    pub created_ts: u64,
}

impl Ord for PostCreatedTsKey {
    // Sort by created ts in descending order
    // Note!!: do reverse compare, since this is a max heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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

impl HasClubId for PostCreatedTsKey {
    fn club_id(&self) -> Option<String> {
        self.club_id.clone()
    }
}

// ######################
// Traits implementations for stable structures
// ######################
const MAX_POST_CREATED_TS_SIZE: u32 = 150; // 50B for post id and 50B for timestamp

impl Storable for PostCreatedTsKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode PostCreatedTs for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode PostCreatedTs from stable storage")
    }
}

impl BoundedStorable for PostCreatedTsKey {
    const MAX_SIZE: u32 = MAX_POST_CREATED_TS_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

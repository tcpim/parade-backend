use crate::models::post_club_model::HasClubId;
use crate::models::post_model::HasPostId;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;

#[derive(PartialEq, Eq, Clone, CandidType, Deserialize, Debug)]
pub struct CollectionPostCreatedTsKey {
    pub canister_id: String,
    pub created_ts: u64,
    pub post_id: String,
    pub club_id: Option<String>,
}

impl Ord for CollectionPostCreatedTsKey {
    // First compare canister id to bucket by collection
    // Then compare post id and club id for equality
    // Sort by created ts in descending order
    // Note!!: do reverse compare on created ts, since this is a max heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.canister_id.cmp(&other.canister_id);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }

        if self.post_id == other.post_id && self.club_id == other.club_id {
            return std::cmp::Ordering::Equal;
        }

        other.created_ts.cmp(&self.created_ts)
    }
}

impl PartialOrd for CollectionPostCreatedTsKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HasPostId for CollectionPostCreatedTsKey {
    fn post_id(&self) -> String {
        self.post_id.clone()
    }
}

impl HasClubId for CollectionPostCreatedTsKey {
    fn club_id(&self) -> Option<String> {
        self.club_id.clone()
    }
}

// ######################
// Traits implementations for stable structures
// ######################
const MAX_CLUB_COLLECTION_CREATED_TS_SIZE: u32 = 500; // 50B for post id and 50B for timestamp and 50B for canister id

impl Storable for CollectionPostCreatedTsKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode ClubPostCreatedTsKey for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode ClubPostCreatedTsKey from stable storage")
    }
}

impl BoundedStorable for CollectionPostCreatedTsKey {
    const MAX_SIZE: u32 = MAX_CLUB_COLLECTION_CREATED_TS_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

use crate::models::post::HasPostId;
use crate::models::trending_post::TrendingPostKey;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;
use crate::models::post_club::HasClubId;

#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize)]
pub struct TrendingPostCollectionKey {
    pub canister_id: String,
    pub trending_info: TrendingPostKey,
}

impl Ord for TrendingPostCollectionKey {
    // First compare canister id
    // Then check trending score, then updated_ts, finally created_ts
    // If same trending score, whoever is being updated (added new reply) is more trending
    // Note!!: do reverse compare, since this is a max heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.canister_id.cmp(&other.canister_id);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }

        self.trending_info.cmp(&other.trending_info)
    }
}

impl PartialOrd for TrendingPostCollectionKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HasPostId for TrendingPostCollectionKey {
    fn post_id(&self) -> String {
        self.trending_info.post_id.clone()
    }
}

impl HasClubId for TrendingPostCollectionKey {
    fn club_id(&self) -> Option<String> {
        self.trending_info.club_id.clone()
    }
}

// ######################
// Traits implementations for stable structures
// ######################
// 50B canister id
// 50B post id
// 4B trending score
// 8B created_ts
// 8B updated_ts
// 120B total
const MAX_TRENDING_POST_COLLECTION_KEY_SIZE: u32 = 200;

impl Storable for TrendingPostCollectionKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode TrendingPostCollectionKey for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self)
            .expect("failed to decode TrendingPostCollectionKey from stable storage")
    }
}

impl BoundedStorable for TrendingPostCollectionKey {
    const MAX_SIZE: u32 = MAX_TRENDING_POST_COLLECTION_KEY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

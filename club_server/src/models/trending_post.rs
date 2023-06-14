use crate::models::post::HasPostId;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;

#[derive(Eq, PartialEq, Clone, Debug, CandidType, Deserialize)]
pub struct TrendingPostKey {
    pub post_id: String,
    pub trending_score: u32,
    pub created_ts: u64,
    pub updated_ts: u64,
}

impl TrendingPostKey {
    pub fn lowest() -> TrendingPostKey {
        TrendingPostKey {
            post_id: "".to_string(),
            trending_score: 0,
            created_ts: 0,
            updated_ts: 0,
        }
    }
}

impl Ord for TrendingPostKey {
    // First check trending score, then updated_ts, finally created_ts
    // If same trending score, whoever is being updated (added new reply) is more trending
    // Note!!: do reverse compare, since this is a max heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = other.trending_score.cmp(&self.trending_score);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }

        let ord = other.updated_ts.cmp(&self.updated_ts);
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }

        other.created_ts.cmp(&self.created_ts)
    }
}

impl PartialOrd for TrendingPostKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HasPostId for TrendingPostKey {
    fn post_id(&self) -> String {
        self.post_id.clone()
    }
}

// ######################
// Traits implementations for stable structures
// ######################
// 50Bytes for post id, 4 bytes for trending score, 16 bytes for created_ts, 16 bytes for updated_ts
const MAX_TRENDING_POST_KEY_SIZE: u32 = 150;

impl Storable for TrendingPostKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode TrendingPostKey for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode TrendingPostKey from stable storage")
    }
}

impl BoundedStorable for TrendingPostKey {
    const MAX_SIZE: u32 = MAX_TRENDING_POST_KEY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;

#[derive(Clone, Debug, CandidType, PartialEq, Deserialize)]
pub struct ClubInfo {
    pub club_id: String,
    pub club_name: String,
    pub club_description: String,
}

// ######################
// Traits implementations for stable structures
// ######################
const MAX_CLUB_INFO_SIZE: u32 = 500; //500 bytes

impl BoundedStorable for ClubInfo {
    const MAX_SIZE: u32 = MAX_CLUB_INFO_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for ClubInfo {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode Post for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode Post from stable storage")
    }
}

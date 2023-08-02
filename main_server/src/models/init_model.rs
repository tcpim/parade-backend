use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, CandidType, Deserialize, Debug)]
pub struct CanisterArgs {
    pub env: String, // either "dev" or "prod"
}

impl BoundedStorable for CanisterArgs {
    const MAX_SIZE: u32 = 10; // 10 bytes for wither "dev" or "prod"
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for CanisterArgs {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode CanisterArgs for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode CanisterArgs from stable storage")
    }
}

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct User {
    pub pid: String, // the user principal string formatxw
    pub user_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct UserPrincipalStringKey(pub String);

// ######################
// Traits implementations for stable structures
// ######################
const MAX_USER_SIZE: u32 = 5000; // 5KB, in case more info to be added
const MAX_USER_PRINCIPAL_STRING_KEY_SIZE: u32 = 150;

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Encode!(self)
            .expect("failed to encode User for stable storage")
            .into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(&bytes, Self).expect("failed to decode User from stable storage")
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = MAX_USER_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for UserPrincipalStringKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }
}

impl BoundedStorable for UserPrincipalStringKey {
    const MAX_SIZE: u32 = MAX_USER_PRINCIPAL_STRING_KEY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

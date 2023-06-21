use candid::{CandidType, Decode, Encode};
use ic_stable_structures::storable::Blob;
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct User {
    pub id: String, // the user principal string
    pub user_name: Option<String>,
    pub avatar: Option<Vec<u8>>,
    pub bio: Option<String>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct UserPrincipalStringKey(pub String);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct UserNameStringKey(pub String);

// ######################
// Traits implementations for stable structures
// ######################
const MAX_USER_SIZE: u32 = 500 * 1000; // 500KB: 200KB for user avatar
const MAX_USER_PRINCIPAL_STRING_KEY_SIZE: u32 = 150;
const MAX_USER_NAME_STRING_KEY: u32 = 100;

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

impl Storable for UserNameStringKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }
}

impl BoundedStorable for UserNameStringKey {
    const MAX_SIZE: u32 = MAX_USER_NAME_STRING_KEY;
    const IS_FIXED_SIZE: bool = false;
}

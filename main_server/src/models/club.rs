use ic_stable_structures::{BoundedStorable, Storable};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct ClubIdString(pub String);

// ######################
// Traits implementations for stable structures
// ######################
const MAX_CLUB_ID_STRING_KEY_SIZE: u32 = 50; // 50Bytes, usually uuid

impl BoundedStorable for ClubIdString {
    const MAX_SIZE: u32 = MAX_CLUB_ID_STRING_KEY_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for ClubIdString {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }
}

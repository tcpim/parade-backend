use candid::CandidType;

use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq)]
pub struct ClubPost {
    pub post_id: String,
    pub club_id: String,
}

pub trait HasClubId {
    fn club_id(&self) -> Option<String>;
}

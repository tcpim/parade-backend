use crate::models::club_model::ClubInfo;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct SetClubInfoRequest {
    pub info: ClubInfo,
}

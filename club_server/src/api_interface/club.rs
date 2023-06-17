use super::common::ServerError;
use crate::models::club::ClubInfo;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct SetClubInfoRequest {
    pub info: ClubInfo,
}

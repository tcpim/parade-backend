use crate::api_interface::common_interface::ServerError;
use crate::models::user_model::User;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct GetUserInfoResponse {
    pub user: Option<User>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct SetUserInfoRequest {
    pub user_id: String,
    pub user_name: Option<String>,
    pub user_avatar: Option<Vec<u8>>,
    pub user_bio: Option<String>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct SetUserInfoResponse {
    pub user: User,
    pub error: Option<ServerError>,
}

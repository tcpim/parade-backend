use crate::api_interface::common_interface::ServerError;
use crate::models::user_model::User;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct GetUserInfoResponse {
    pub user: Option<User>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct SetUserAvatarRequest {
    pub user_id: String,
    pub avatar: Vec<u8>,
    pub mime_type: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct SetUserNameRequest {
    pub user_id: String,
    pub new_name: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct SetUserBioRequest {
    pub user_id: String,
    pub bio: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct SetUserInfoResponse {
    pub user: User,
    pub error: Option<ServerError>,
}

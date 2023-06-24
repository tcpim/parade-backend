use crate::models::post_model::Post;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize, PartialEq)]
pub struct ServerError {
    pub api_name: String,
    pub error_message: String,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct PostList {
    pub offset: i32,
    pub posts: Vec<Post>,
    pub error: Option<ServerError>,
}

// When cursor is None
// 1. if used in api request, fetch from the start
// 2. if used in api response, there is no more data
#[derive(Debug, CandidType, Deserialize, Eq, PartialEq, Clone)]
pub struct Cursor<T>(pub Option<T>);

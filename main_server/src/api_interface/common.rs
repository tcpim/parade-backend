use crate::models::post::Post;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize, PartialEq)]
pub enum ServerError {
    CreatePostGeneralError(String),
    GetPostError(String),
    GetPostByUserError(String),
    GetPostByCollectionError(String),
    GetStreetPostsError(String),
    DeletePostError(String),
    ReplyPostError(String),
    ReactEmojiError(String),
    GetPostRepliesError(String),
    GetTrendingPostsError(String),
    SetUserInfoError(String),
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

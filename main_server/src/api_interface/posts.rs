use super::common::ServerError;
use crate::api_interface::common::Cursor;
use crate::models::nft::NftToken;
use crate::models::post::Post;
use crate::models::post_club::{ClubPost};
use crate::models::post_collection::CollectionPostCreatedTsKey;
use crate::models::post_street::PostCreatedTsKey;
use crate::models::post_user::UserPostCreatedTsKey;
use candid::CandidType;
use serde::Deserialize;

// Only one type of post is allowed
#[derive(Debug, CandidType, Deserialize)]
pub struct PostType {
    pub post: Option<Post>,
    pub club_post: Option<ClubPost>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct CreateStreetPostRequest {
    pub post_id: String,    // the post id, uuid from frontend
    pub created_by: String, // User's principal
    pub nfts: Vec<NftToken>,
    pub words: String,
    pub created_ts: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct CreateStreetPostResponse {
    pub post: Post,
    pub error: Option<ServerError>, // if set, there is error and post should be ignored
}

#[derive(Debug, CandidType, Deserialize)]
pub struct AddClubPostToStreetRequest {
    pub post_id: String,
    pub club_id: String,
    pub nfts: Vec<NftToken>,
    pub created_ts: u64,
    pub created_by: String,
}


#[derive(Debug, CandidType, Deserialize)]
pub struct GetStreetPostsRequest {
    pub limit: Option<i32>,
    pub cursor: Cursor<PostCreatedTsKey>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetStreetPostsResponse {
    pub posts: Vec<PostType>,
    pub next_cursor: Cursor<PostCreatedTsKey>,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetPostByIdResponse {
    pub post: Option<Post>,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetUserPostsRequest {
    pub user_id: String, // the user principal
    pub cursor: Cursor<UserPostCreatedTsKey>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetUserPostsResponse {
    pub posts: Vec<PostType>,
    pub next_cursor: Cursor<UserPostCreatedTsKey>,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct UserAddPostRequest {
    pub user_id: String, // the user principal
    pub post_id: String,
    pub club_id: Option<String>,
    pub created_ts: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetCollectionPostsRequest {
    pub canister_id: String, //
    pub cursor: Cursor<CollectionPostCreatedTsKey>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetCollectionPostsResponse {
    pub posts: Vec<PostType>,
    pub next_cursor: Cursor<CollectionPostCreatedTsKey>,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct DeletePostResponse {
    pub error: Option<ServerError>,
}
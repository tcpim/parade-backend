use super::common::ServerError;
use crate::api_interface::common::Cursor;
use crate::models::nft::NftToken;
use crate::models::post::Post;
use crate::models::post::PostCreatedTsKey;
use crate::models::post_collection::CollectionPostCreatedTsKey;
use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct CreatePostRequest {
    pub post_id: String,    // the post id, uuid from frontend
    pub created_by: String, // User's principal
    pub nfts: Vec<NftToken>,
    pub in_public: bool, // whether this post is viewable in public street
    pub club_id: String, // whether this post is viewable in NFT clubs, if set, store club id list
    pub words: String,
    pub created_ts: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct CreatePostResponse {
    pub post: Post,
    pub error: Option<ServerError>, // if set, there is error and post should be ignored
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetPostByIdResponse {
    pub post: Vec<Post>,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetCollectionPostsRequest {
    pub canister_id: String, //
    pub cursor: Cursor<CollectionPostCreatedTsKey>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetCollectionPostsResponse {
    pub posts: Vec<Post>,
    pub next_cursor: Cursor<CollectionPostCreatedTsKey>,
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct DeletePostResponse {
    pub error: Option<ServerError>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetPostsRequest {
    pub cursor: Cursor<PostCreatedTsKey>,
    pub limit: Option<i32>,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct GetPostsResponse {
    pub posts: Vec<Post>,
    pub next_cursor: Cursor<PostCreatedTsKey>,
    pub error: Option<ServerError>,
}

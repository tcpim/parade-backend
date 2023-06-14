import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ClubPostCreatedTsKey {
  'post_id' : string,
  'created_ts' : bigint,
  'club_id' : string,
}
export interface CollectionPostCreatedTsKey {
  'post_id' : string,
  'canister_id' : string,
  'created_ts' : bigint,
}
export interface CreatePostRequest {
  'post_id' : string,
  'club_ids' : [] | [Array<string>],
  'nfts' : Array<NftToken>,
  'created_by' : string,
  'created_ts' : bigint,
  'words' : string,
  'in_public' : boolean,
}
export interface CreatePostResponse {
  'post' : Post,
  'error' : [] | [ServerError],
}
export interface DeletePostResponse { 'error' : [] | [ServerError] }
export interface GetClubPostsRequest {
  'cursor' : [] | [ClubPostCreatedTsKey],
  'limit' : [] | [number],
  'club_id' : string,
}
export interface GetClubPostsResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [ClubPostCreatedTsKey],
  'posts' : Array<Post>,
}
export interface GetCollectionPostsRequest {
  'cursor' : [] | [CollectionPostCreatedTsKey],
  'canister_id' : string,
  'limit' : [] | [number],
}
export interface GetCollectionPostsResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [CollectionPostCreatedTsKey],
  'posts' : Array<Post>,
}
export interface GetPostByIdResponse {
  'post' : [] | [Post],
  'error' : [] | [ServerError],
}
export interface GetPostRepliesRequest {
  'post_id' : string,
  'offset' : number,
  'limit' : [] | [number],
}
export interface GetPostRepliesResponse {
  'offset' : number,
  'error' : [] | [ServerError],
  'post_replies' : Array<PostReply>,
}
export interface GetStreetPostsRequest {
  'cursor' : [] | [PostCreatedTsKey],
  'limit' : [] | [number],
}
export interface GetStreetPostsResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [PostCreatedTsKey],
  'posts' : Array<Post>,
}
export interface GetTrendingClubPostRequest {
  'cursor' : [] | [TrendingPostClubKey],
  'limit' : [] | [number],
  'club_id' : string,
}
export interface GetTrendingClubPostResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [TrendingPostClubKey],
  'posts' : Array<Post>,
}
export interface GetTrendingCollectionPostRequest {
  'cursor' : [] | [TrendingPostCollectionKey],
  'canister_id' : string,
  'limit' : [] | [number],
}
export interface GetTrendingCollectionPostResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [TrendingPostCollectionKey],
  'posts' : Array<Post>,
}
export interface GetTrendingStreetPostRequest {
  'cursor' : [] | [TrendingPostKey],
  'limit' : [] | [number],
}
export interface GetTrendingStreetPostResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [TrendingPostKey],
  'posts' : Array<Post>,
}
export interface GetUserPostsRequest {
  'cursor' : [] | [UserPostCreatedTsKey],
  'user_id' : string,
  'limit' : [] | [number],
}
export interface GetUserPostsResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [UserPostCreatedTsKey],
  'posts' : Array<Post>,
}
export interface NftToken {
  'token_index' : number,
  'token_id' : string,
  'canister_id' : string,
  'collection_name' : string,
  'original_image_url' : string,
  'original_thumbnail_url' : string,
}
export interface Post {
  'id' : string,
  'updated_ts' : [] | [bigint],
  'club_ids' : [] | [Array<string>],
  'emoji_reactions' : Array<[string, number]>,
  'nfts' : Array<NftToken>,
  'created_by' : string,
  'created_ts' : bigint,
  'replies' : Array<string>,
  'words' : string,
  'in_public' : boolean,
  'trending_score' : [] | [number],
}
export interface PostCreatedTsKey { 'post_id' : string, 'created_ts' : bigint }
export interface PostReply {
  'id' : string,
  'post_id' : string,
  'emoji_reactions' : Array<[string, number]>,
  'nfts' : Array<NftToken>,
  'created_by' : string,
  'created_ts' : bigint,
  'words' : string,
}
export interface ReactEmojiRequest {
  'post_id' : [] | [string],
  'reply_id' : [] | [string],
  'user' : string,
  'created_ts' : bigint,
  'emoji' : string,
}
export interface ReplyPostRequest {
  'post_id' : string,
  'reply_id' : string,
  'nfts' : Array<NftToken>,
  'user' : string,
  'created_ts' : bigint,
  'words' : string,
}
export interface ReplyPostResponse {
  'error' : [] | [ServerError],
  'reply' : PostReply,
}
export type ServerError = { 'GetPostError' : string } |
  { 'GetTrendingPostsError' : string } |
  { 'ReactEmojiError' : string } |
  { 'CreatePostGeneralError' : string } |
  { 'GetPostByCollectionError' : string } |
  { 'ReplyPostError' : string } |
  { 'GetStreetPostsError' : string } |
  { 'GetPostRepliesError' : string } |
  { 'DeletePostError' : string } |
  { 'GetPostByUserError' : string };
export interface TrendingPostClubKey {
  'trending_info' : TrendingPostKey,
  'club_id' : string,
}
export interface TrendingPostCollectionKey {
  'trending_info' : TrendingPostKey,
  'canister_id' : string,
}
export interface TrendingPostKey {
  'updated_ts' : bigint,
  'post_id' : string,
  'created_ts' : bigint,
  'trending_score' : number,
}
export interface UserPostCreatedTsKey {
  'post_id' : string,
  'created_ts' : bigint,
  'user_id' : string,
}
export interface _SERVICE {
  'create_post' : ActorMethod<[CreatePostRequest], CreatePostResponse>,
  'delete_all_post' : ActorMethod<[], undefined>,
  'delete_post' : ActorMethod<[string], DeletePostResponse>,
  'get_post_by_id' : ActorMethod<[string], GetPostByIdResponse>,
  'get_post_replies' : ActorMethod<
    [GetPostRepliesRequest],
    GetPostRepliesResponse
  >,
  'get_posts_by_club' : ActorMethod<
    [GetClubPostsRequest],
    GetClubPostsResponse
  >,
  'get_posts_by_collection' : ActorMethod<
    [GetCollectionPostsRequest],
    GetCollectionPostsResponse
  >,
  'get_posts_by_user' : ActorMethod<
    [GetUserPostsRequest],
    GetUserPostsResponse
  >,
  'get_street_posts' : ActorMethod<
    [GetStreetPostsRequest],
    GetStreetPostsResponse
  >,
  'get_trending_club_posts' : ActorMethod<
    [GetTrendingClubPostRequest],
    GetTrendingClubPostResponse
  >,
  'get_trending_collection_posts' : ActorMethod<
    [GetTrendingCollectionPostRequest],
    GetTrendingCollectionPostResponse
  >,
  'get_trending_street_posts' : ActorMethod<
    [GetTrendingStreetPostRequest],
    GetTrendingStreetPostResponse
  >,
  'react_emoji' : ActorMethod<[ReactEmojiRequest], DeletePostResponse>,
  'reply_post' : ActorMethod<[ReplyPostRequest], ReplyPostResponse>,
}

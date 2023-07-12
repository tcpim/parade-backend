import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ChatClubMessage {
  'id' : string,
  'updated_ts' : bigint,
  'emoji_reactions' : Array<[string, number]>,
  'created_ts' : bigint,
  'user_id' : string,
  'words' : string,
}
export interface ClubInfo {
  'club_description' : string,
  'club_name' : string,
  'club_id' : string,
}
export interface CollectionPostCreatedTsKey {
  'post_id' : string,
  'canister_id' : string,
  'created_ts' : bigint,
}
export interface CreatePostRequest {
  'post_id' : string,
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
export interface DeleteClubMessageRequest {
  'deleter' : string,
  'deleted_ts' : bigint,
  'message_id' : string,
}
export interface DeletePostResponse { 'error' : [] | [ServerError] }
export interface GetClubMessagesRequest {
  'cursor' : [] | [bigint],
  'limit' : [] | [number],
}
export interface GetClubMessagesResponse {
  'messages' : Array<ChatClubMessage>,
  'error' : [] | [ServerError],
  'next_cursor' : [] | [bigint],
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
export interface GetPostByIdResponse { 'post' : [] | [Post] }
export interface GetPostByIdsResponse { 'posts' : Array<[] | [Post]> }
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
export interface GetPostsRequest {
  'cursor' : [] | [PostCreatedTsKey],
  'limit' : [] | [number],
}
export interface GetPostsResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [PostCreatedTsKey],
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
export interface GetTrendingPostRequest {
  'cursor' : [] | [TrendingPostKey],
  'limit' : [] | [number],
}
export interface GetTrendingPostResponse {
  'error' : [] | [ServerError],
  'next_cursor' : [] | [TrendingPostKey],
  'posts' : Array<Post>,
}
export interface NftToken {
  'image_height_width_ratio' : string,
  'token_index' : number,
  'token_id' : string,
  'image_url' : string,
  'image_thumbnail_url' : string,
  'canister_id' : string,
  'collection_name' : string,
  'image_type' : string,
  'image_onchain_url' : string,
}
export interface Post {
  'id' : string,
  'updated_ts' : bigint,
  'emoji_reactions' : Array<[string, number]>,
  'nfts' : Array<NftToken>,
  'created_by' : string,
  'created_ts' : bigint,
  'replies' : Array<string>,
  'words' : string,
  'club_id' : string,
  'in_public' : boolean,
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
export interface ReactClubMessageRequest {
  'emoji' : string,
  'message_id' : string,
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
export interface SendClubMessageRequest {
  'created_ts' : bigint,
  'sender' : string,
  'words' : string,
  'message_id' : string,
}
export interface ServerError { 'error_message' : string, 'api_name' : string }
export interface SetClubInfoRequest { 'info' : ClubInfo }
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
export interface UpdateClubMessageRequest {
  'updated_ts' : bigint,
  'updater' : string,
  'words' : string,
  'message_id' : string,
}
export interface _SERVICE {
  'create_post' : ActorMethod<[CreatePostRequest], CreatePostResponse>,
  'delete_all_club_message' : ActorMethod<[], undefined>,
  'delete_all_post' : ActorMethod<[], undefined>,
  'delete_club_message' : ActorMethod<
    [DeleteClubMessageRequest],
    [] | [ServerError]
  >,
  'delete_post' : ActorMethod<[string], DeletePostResponse>,
  'get_club_info' : ActorMethod<[], ClubInfo>,
  'get_club_message_by_id' : ActorMethod<[string], [] | [ChatClubMessage]>,
  'get_club_messages' : ActorMethod<
    [GetClubMessagesRequest],
    GetClubMessagesResponse
  >,
  'get_post_by_id' : ActorMethod<[string], GetPostByIdResponse>,
  'get_post_by_ids' : ActorMethod<[Array<string>], GetPostByIdsResponse>,
  'get_post_replies' : ActorMethod<
    [GetPostRepliesRequest],
    GetPostRepliesResponse
  >,
  'get_posts' : ActorMethod<[GetPostsRequest], GetPostsResponse>,
  'get_posts_by_collection' : ActorMethod<
    [GetCollectionPostsRequest],
    GetCollectionPostsResponse
  >,
  'get_trending_collection_posts' : ActorMethod<
    [GetTrendingCollectionPostRequest],
    GetTrendingCollectionPostResponse
  >,
  'get_trending_posts' : ActorMethod<
    [GetTrendingPostRequest],
    GetTrendingPostResponse
  >,
  'react_club_message' : ActorMethod<[ReactClubMessageRequest], undefined>,
  'react_emoji' : ActorMethod<[ReactEmojiRequest], DeletePostResponse>,
  'reply_post' : ActorMethod<[ReplyPostRequest], ReplyPostResponse>,
  'send_club_message' : ActorMethod<[SendClubMessageRequest], undefined>,
  'set_club_info' : ActorMethod<[SetClubInfoRequest], undefined>,
  'update_club_message' : ActorMethod<
    [UpdateClubMessageRequest],
    [] | [ServerError]
  >,
}

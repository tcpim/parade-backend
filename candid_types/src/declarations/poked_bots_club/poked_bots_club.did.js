export const idlFactory = ({ IDL }) => {
  const ClubInfo = IDL.Record({
    'club_description' : IDL.Text,
    'club_name' : IDL.Text,
    'club_id' : IDL.Text,
  });
  const SetClubInfoRequest = IDL.Record({ 'info' : ClubInfo });
  const NftToken = IDL.Record({
    'token_index' : IDL.Nat16,
    'token_id' : IDL.Text,
    'canister_id' : IDL.Text,
    'collection_name' : IDL.Text,
    'original_image_url' : IDL.Text,
    'original_thumbnail_url' : IDL.Text,
  });
  const CreatePostRequest = IDL.Record({
    'post_id' : IDL.Text,
    'nfts' : IDL.Vec(NftToken),
    'created_by' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'words' : IDL.Text,
    'in_public' : IDL.Bool,
  });
  const Post = IDL.Record({
    'id' : IDL.Text,
    'updated_ts' : IDL.Nat64,
    'emoji_reactions' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)),
    'nfts' : IDL.Vec(NftToken),
    'created_by' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'replies' : IDL.Vec(IDL.Text),
    'words' : IDL.Text,
    'club_id' : IDL.Text,
    'in_public' : IDL.Bool,
  });
  const ServerError = IDL.Record({
    'error_message' : IDL.Text,
    'api_name' : IDL.Text,
  });
  const CreatePostResponse = IDL.Record({
    'post' : Post,
    'error' : IDL.Opt(ServerError),
  });
  const DeleteClubMessageRequest = IDL.Record({
    'deleter' : IDL.Text,
    'deleted_ts' : IDL.Nat64,
    'message_id' : IDL.Text,
  });
  const DeletePostResponse = IDL.Record({ 'error' : IDL.Opt(ServerError) });
  const ChatClubMessage = IDL.Record({
    'id' : IDL.Text,
    'updated_ts' : IDL.Nat64,
    'emoji_reactions' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)),
    'created_ts' : IDL.Nat64,
    'user_id' : IDL.Text,
    'words' : IDL.Text,
  });
  const GetClubMessagesRequest = IDL.Record({
    'cursor' : IDL.Opt(IDL.Nat64),
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetClubMessagesResponse = IDL.Record({
    'messages' : IDL.Vec(ChatClubMessage),
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(IDL.Nat64),
  });
  const GetPostByIdResponse = IDL.Record({ 'post' : IDL.Opt(Post) });
  const GetPostByIdsResponse = IDL.Record({ 'posts' : IDL.Vec(IDL.Opt(Post)) });
  const GetPostRepliesRequest = IDL.Record({
    'post_id' : IDL.Text,
    'offset' : IDL.Int32,
    'limit' : IDL.Opt(IDL.Int32),
  });
  const PostReply = IDL.Record({
    'id' : IDL.Text,
    'post_id' : IDL.Text,
    'emoji_reactions' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)),
    'nfts' : IDL.Vec(NftToken),
    'created_by' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'words' : IDL.Text,
  });
  const GetPostRepliesResponse = IDL.Record({
    'offset' : IDL.Int32,
    'error' : IDL.Opt(ServerError),
    'post_replies' : IDL.Vec(PostReply),
  });
  const PostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
  });
  const GetPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(PostCreatedTsKey),
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(PostCreatedTsKey),
    'posts' : IDL.Vec(Post),
  });
  const CollectionPostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'canister_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
  });
  const GetCollectionPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(CollectionPostCreatedTsKey),
    'canister_id' : IDL.Text,
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetCollectionPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(CollectionPostCreatedTsKey),
    'posts' : IDL.Vec(Post),
  });
  const TrendingPostKey = IDL.Record({
    'updated_ts' : IDL.Nat64,
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'trending_score' : IDL.Nat32,
  });
  const TrendingPostCollectionKey = IDL.Record({
    'trending_info' : TrendingPostKey,
    'canister_id' : IDL.Text,
  });
  const GetTrendingCollectionPostRequest = IDL.Record({
    'cursor' : IDL.Opt(TrendingPostCollectionKey),
    'canister_id' : IDL.Text,
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetTrendingCollectionPostResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(TrendingPostCollectionKey),
    'posts' : IDL.Vec(Post),
  });
  const GetTrendingPostRequest = IDL.Record({
    'cursor' : IDL.Opt(TrendingPostKey),
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetTrendingPostResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(TrendingPostKey),
    'posts' : IDL.Vec(Post),
  });
  const ReactClubMessageRequest = IDL.Record({
    'emoji' : IDL.Text,
    'message_id' : IDL.Text,
  });
  const ReactEmojiRequest = IDL.Record({
    'post_id' : IDL.Opt(IDL.Text),
    'reply_id' : IDL.Opt(IDL.Text),
    'user' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'emoji' : IDL.Text,
  });
  const ReplyPostRequest = IDL.Record({
    'post_id' : IDL.Text,
    'reply_id' : IDL.Text,
    'nfts' : IDL.Vec(NftToken),
    'user' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'words' : IDL.Text,
  });
  const ReplyPostResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'reply' : PostReply,
  });
  const SendClubMessageRequest = IDL.Record({
    'created_ts' : IDL.Nat64,
    'sender' : IDL.Text,
    'words' : IDL.Text,
    'message_id' : IDL.Text,
  });
  const UpdateClubMessageRequest = IDL.Record({
    'updated_ts' : IDL.Nat64,
    'updater' : IDL.Text,
    'words' : IDL.Text,
    'message_id' : IDL.Text,
  });
  return IDL.Service({
    'create_post' : IDL.Func([CreatePostRequest], [CreatePostResponse], []),
    'delete_all_club_message' : IDL.Func([], [], []),
    'delete_all_post' : IDL.Func([], [], []),
    'delete_club_message' : IDL.Func(
        [DeleteClubMessageRequest],
        [IDL.Opt(ServerError)],
        [],
      ),
    'delete_post' : IDL.Func([IDL.Text], [DeletePostResponse], []),
    'get_club_info' : IDL.Func([], [ClubInfo], ['query']),
    'get_club_message_by_id' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(ChatClubMessage)],
        ['query'],
      ),
    'get_club_messages' : IDL.Func(
        [GetClubMessagesRequest],
        [GetClubMessagesResponse],
        ['query'],
      ),
    'get_post_by_id' : IDL.Func([IDL.Text], [GetPostByIdResponse], ['query']),
    'get_post_by_ids' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [GetPostByIdsResponse],
        ['query'],
      ),
    'get_post_replies' : IDL.Func(
        [GetPostRepliesRequest],
        [GetPostRepliesResponse],
        ['query'],
      ),
    'get_posts' : IDL.Func([GetPostsRequest], [GetPostsResponse], ['query']),
    'get_posts_by_collection' : IDL.Func(
        [GetCollectionPostsRequest],
        [GetCollectionPostsResponse],
        ['query'],
      ),
    'get_trending_collection_posts' : IDL.Func(
        [GetTrendingCollectionPostRequest],
        [GetTrendingCollectionPostResponse],
        ['query'],
      ),
    'get_trending_posts' : IDL.Func(
        [GetTrendingPostRequest],
        [GetTrendingPostResponse],
        ['query'],
      ),
    'react_club_message' : IDL.Func([ReactClubMessageRequest], [], []),
    'react_emoji' : IDL.Func([ReactEmojiRequest], [DeletePostResponse], []),
    'reply_post' : IDL.Func([ReplyPostRequest], [ReplyPostResponse], []),
    'send_club_message' : IDL.Func([SendClubMessageRequest], [], []),
    'set_club_info' : IDL.Func([SetClubInfoRequest], [], []),
    'update_club_message' : IDL.Func(
        [UpdateClubMessageRequest],
        [IDL.Opt(ServerError)],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const ClubInfo = IDL.Record({
    'club_description' : IDL.Text,
    'club_name' : IDL.Text,
    'club_id' : IDL.Text,
  });
  const SetClubInfoRequest = IDL.Record({ 'info' : ClubInfo });
  return [SetClubInfoRequest];
};

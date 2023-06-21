export const idlFactory = ({ IDL }) => {
  const NftToken = IDL.Record({
    'token_index' : IDL.Nat16,
    'token_id' : IDL.Text,
    'canister_id' : IDL.Text,
    'collection_name' : IDL.Text,
    'original_image_url' : IDL.Text,
    'original_thumbnail_url' : IDL.Text,
  });
  const AddClubPostToStreetRequest = IDL.Record({
    'post_id' : IDL.Text,
    'nfts' : IDL.Vec(NftToken),
    'created_by' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'club_id' : IDL.Text,
  });
  const UserPostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'user_id' : IDL.Text,
    'club_id' : IDL.Opt(IDL.Text),
  });
  const CreateStreetPostRequest = IDL.Record({
    'post_id' : IDL.Text,
    'nfts' : IDL.Vec(NftToken),
    'created_by' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'words' : IDL.Text,
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
  });
  const ServerError = IDL.Variant({
    'GetPostError' : IDL.Text,
    'GetTrendingPostsError' : IDL.Text,
    'SetUserInfoError' : IDL.Text,
    'ReactEmojiError' : IDL.Text,
    'CreatePostGeneralError' : IDL.Text,
    'GetPostByCollectionError' : IDL.Text,
    'ReplyPostError' : IDL.Text,
    'GetStreetPostsError' : IDL.Text,
    'GetPostRepliesError' : IDL.Text,
    'DeletePostError' : IDL.Text,
    'GetPostByUserError' : IDL.Text,
  });
  const CreateStreetPostResponse = IDL.Record({
    'post' : Post,
    'error' : IDL.Opt(ServerError),
  });
  const DeletePostResponse = IDL.Record({ 'error' : IDL.Opt(ServerError) });
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
  const CollectionPostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'canister_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'club_id' : IDL.Opt(IDL.Text),
  });
  const GetCollectionPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(CollectionPostCreatedTsKey),
    'canister_id' : IDL.Text,
    'limit' : IDL.Opt(IDL.Int32),
  });
  const ClubPost = IDL.Record({ 'post_id' : IDL.Text, 'club_id' : IDL.Text });
  const PostType = IDL.Record({
    'post' : IDL.Opt(Post),
    'club_post' : IDL.Opt(ClubPost),
  });
  const GetCollectionPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(CollectionPostCreatedTsKey),
    'posts' : IDL.Vec(PostType),
  });
  const GetUserPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(UserPostCreatedTsKey),
    'user_id' : IDL.Text,
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetUserPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(UserPostCreatedTsKey),
    'posts' : IDL.Vec(PostType),
  });
  const GetPostByIdResponse = IDL.Record({ 'post' : IDL.Opt(Post) });
  const PostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'club_id' : IDL.Opt(IDL.Text),
  });
  const GetStreetPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(PostCreatedTsKey),
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetStreetPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(PostCreatedTsKey),
    'posts' : IDL.Vec(PostType),
  });
  const TrendingPostKey = IDL.Record({
    'updated_ts' : IDL.Nat64,
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'club_id' : IDL.Opt(IDL.Text),
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
    'posts' : IDL.Vec(PostType),
  });
  const GetTrendingStreetPostRequest = IDL.Record({
    'cursor' : IDL.Opt(TrendingPostKey),
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetTrendingStreetPostResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(TrendingPostKey),
    'posts' : IDL.Vec(PostType),
  });
  const User = IDL.Record({
    'id' : IDL.Text,
    'bio' : IDL.Opt(IDL.Text),
    'user_name' : IDL.Opt(IDL.Text),
    'avatar' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const GetUserInfoResponse = IDL.Record({ 'user' : IDL.Opt(User) });
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
  const SetUserInfoRequest = IDL.Record({
    'user_name' : IDL.Opt(IDL.Text),
    'user_bio' : IDL.Opt(IDL.Text),
    'user_avatar' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'user_id' : IDL.Text,
  });
  const SetUserInfoResponse = IDL.Record({
    'user' : User,
    'error' : IDL.Opt(ServerError),
  });
  const UpdateClubPostStreetTrendingScoreRequest = IDL.Record({
    'new' : TrendingPostKey,
    'nft_canister_ids' : IDL.Vec(IDL.Text),
  });
  return IDL.Service({
    'add_club_post_to_street' : IDL.Func([AddClubPostToStreetRequest], [], []),
    'add_club_post_to_user' : IDL.Func([UserPostCreatedTsKey], [], []),
    'create_street_post' : IDL.Func(
        [CreateStreetPostRequest],
        [CreateStreetPostResponse],
        [],
      ),
    'create_user' : IDL.Func([IDL.Text], [], []),
    'delete_all_post' : IDL.Func([], [], []),
    'delete_post' : IDL.Func([IDL.Text], [DeletePostResponse], []),
    'get_post_replies' : IDL.Func(
        [GetPostRepliesRequest],
        [GetPostRepliesResponse],
        ['query'],
      ),
    'get_posts_by_collection' : IDL.Func(
        [GetCollectionPostsRequest],
        [GetCollectionPostsResponse],
        ['query'],
      ),
    'get_posts_by_user' : IDL.Func(
        [GetUserPostsRequest],
        [GetUserPostsResponse],
        ['query'],
      ),
    'get_street_post_by_id' : IDL.Func(
        [IDL.Text],
        [GetPostByIdResponse],
        ['query'],
      ),
    'get_street_posts' : IDL.Func(
        [GetStreetPostsRequest],
        [GetStreetPostsResponse],
        ['query'],
      ),
    'get_trending_collection_posts' : IDL.Func(
        [GetTrendingCollectionPostRequest],
        [GetTrendingCollectionPostResponse],
        ['query'],
      ),
    'get_trending_street_posts' : IDL.Func(
        [GetTrendingStreetPostRequest],
        [GetTrendingStreetPostResponse],
        ['query'],
      ),
    'get_user_info' : IDL.Func([IDL.Text], [GetUserInfoResponse], ['query']),
    'react_emoji' : IDL.Func([ReactEmojiRequest], [DeletePostResponse], []),
    'reply_post' : IDL.Func([ReplyPostRequest], [ReplyPostResponse], []),
    'set_user_info' : IDL.Func([SetUserInfoRequest], [SetUserInfoResponse], []),
    'update_club_post_trending_score' : IDL.Func(
        [UpdateClubPostStreetTrendingScoreRequest],
        [],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };

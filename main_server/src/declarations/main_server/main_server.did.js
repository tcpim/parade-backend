export const idlFactory = ({ IDL }) => {
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
    'club_ids' : IDL.Opt(IDL.Vec(IDL.Text)),
    'nfts' : IDL.Vec(NftToken),
    'created_by' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'words' : IDL.Text,
    'in_public' : IDL.Bool,
  });
  const Post = IDL.Record({
    'id' : IDL.Text,
    'updated_ts' : IDL.Opt(IDL.Nat64),
    'club_ids' : IDL.Opt(IDL.Vec(IDL.Text)),
    'emoji_reactions' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)),
    'nfts' : IDL.Vec(NftToken),
    'created_by' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'replies' : IDL.Vec(IDL.Text),
    'words' : IDL.Text,
    'in_public' : IDL.Bool,
    'trending_score' : IDL.Opt(IDL.Nat32),
  });
  const ServerError = IDL.Variant({
    'GetPostError' : IDL.Text,
    'GetTrendingPostsError' : IDL.Text,
    'ReactEmojiError' : IDL.Text,
    'CreatePostGeneralError' : IDL.Text,
    'GetPostByCollectionError' : IDL.Text,
    'ReplyPostError' : IDL.Text,
    'GetStreetPostsError' : IDL.Text,
    'GetPostRepliesError' : IDL.Text,
    'DeletePostError' : IDL.Text,
    'GetPostByUserError' : IDL.Text,
  });
  const CreatePostResponse = IDL.Record({
    'post' : Post,
    'error' : IDL.Opt(ServerError),
  });
  const DeletePostResponse = IDL.Record({ 'error' : IDL.Opt(ServerError) });
  const GetPostByIdResponse = IDL.Record({
    'post' : IDL.Opt(Post),
    'error' : IDL.Opt(ServerError),
  });
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
  const ClubPostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'club_id' : IDL.Text,
  });
  const GetClubPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(ClubPostCreatedTsKey),
    'limit' : IDL.Opt(IDL.Int32),
    'club_id' : IDL.Text,
  });
  const GetClubPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(ClubPostCreatedTsKey),
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
  const UserPostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'user_id' : IDL.Text,
  });
  const GetUserPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(UserPostCreatedTsKey),
    'user_id' : IDL.Text,
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetUserPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(UserPostCreatedTsKey),
    'posts' : IDL.Vec(Post),
  });
  const PostCreatedTsKey = IDL.Record({
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
  });
  const GetStreetPostsRequest = IDL.Record({
    'cursor' : IDL.Opt(PostCreatedTsKey),
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetStreetPostsResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(PostCreatedTsKey),
    'posts' : IDL.Vec(Post),
  });
  const TrendingPostKey = IDL.Record({
    'updated_ts' : IDL.Nat64,
    'post_id' : IDL.Text,
    'created_ts' : IDL.Nat64,
    'trending_score' : IDL.Nat32,
  });
  const TrendingPostClubKey = IDL.Record({
    'trending_info' : TrendingPostKey,
    'club_id' : IDL.Text,
  });
  const GetTrendingClubPostRequest = IDL.Record({
    'cursor' : IDL.Opt(TrendingPostClubKey),
    'limit' : IDL.Opt(IDL.Int32),
    'club_id' : IDL.Text,
  });
  const GetTrendingClubPostResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(TrendingPostClubKey),
    'posts' : IDL.Vec(Post),
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
  const GetTrendingStreetPostRequest = IDL.Record({
    'cursor' : IDL.Opt(TrendingPostKey),
    'limit' : IDL.Opt(IDL.Int32),
  });
  const GetTrendingStreetPostResponse = IDL.Record({
    'error' : IDL.Opt(ServerError),
    'next_cursor' : IDL.Opt(TrendingPostKey),
    'posts' : IDL.Vec(Post),
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
  return IDL.Service({
    'create_post' : IDL.Func([CreatePostRequest], [CreatePostResponse], []),
    'delete_all_post' : IDL.Func([], [], []),
    'delete_post' : IDL.Func([IDL.Text], [DeletePostResponse], []),
    'get_post_by_id' : IDL.Func([IDL.Text], [GetPostByIdResponse], ['query']),
    'get_post_replies' : IDL.Func(
        [GetPostRepliesRequest],
        [GetPostRepliesResponse],
        ['query'],
      ),
    'get_posts_by_club' : IDL.Func(
        [GetClubPostsRequest],
        [GetClubPostsResponse],
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
    'get_street_posts' : IDL.Func(
        [GetStreetPostsRequest],
        [GetStreetPostsResponse],
        ['query'],
      ),
    'get_trending_club_posts' : IDL.Func(
        [GetTrendingClubPostRequest],
        [GetTrendingClubPostResponse],
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
    'react_emoji' : IDL.Func([ReactEmojiRequest], [DeletePostResponse], []),
    'reply_post' : IDL.Func([ReplyPostRequest], [ReplyPostResponse], []),
  });
};
export const init = ({ IDL }) => { return []; };

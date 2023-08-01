use super::storages::*;

pub fn with_post_by_id<R>(f: impl FnOnce(&PostByIdMap) -> R) -> R {
    POST_BY_ID.with(|s| f(&s.borrow()))
}

pub fn with_post_by_id_mut<R>(f: impl FnOnce(&mut PostByIdMap) -> R) -> R {
    POST_BY_ID.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_post_reply_by_id<R>(f: impl FnOnce(&PostReplyByIdMap) -> R) -> R {
    POST_REPLY_BY_ID.with(|s| f(&s.borrow()))
}

pub fn with_post_reply_by_id_mut<R>(f: impl FnOnce(&mut PostReplyByIdMap) -> R) -> R {
    POST_REPLY_BY_ID.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_club_posts_created<R>(f: impl FnOnce(&PostsCreatedHeap) -> R) -> R {
    CLUB_POSTS_CREATED.with(|s| f(&s.borrow()))
}

pub fn with_club_posts_created_mut<R>(f: impl FnOnce(&mut PostsCreatedHeap) -> R) -> R {
    CLUB_POSTS_CREATED.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_collection_posts_created<R>(f: impl FnOnce(&CollectionPostsCreatedHeap) -> R) -> R {
    COLLECTION_POSTS_CREATED.with(|s| f(&s.borrow()))
}

pub fn with_collection_posts_created_mut<R>(
    f: impl FnOnce(&mut CollectionPostsCreatedHeap) -> R,
) -> R {
    COLLECTION_POSTS_CREATED.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_trending_posts<R>(f: impl FnOnce(&TrendingPostClubHeap) -> R) -> R {
    TRENDING_POSTS.with(|s| f(&s.borrow()))
}

pub fn with_trending_posts_mut<R>(f: impl FnOnce(&mut TrendingPostClubHeap) -> R) -> R {
    TRENDING_POSTS.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_trending_posts_collection<R>(f: impl FnOnce(&TrendingPostCollectionHeap) -> R) -> R {
    TRENDING_POSTS_COLLECTION.with(|s| f(&s.borrow()))
}

pub fn with_trending_posts_collection_mut<R>(
    f: impl FnOnce(&mut TrendingPostCollectionHeap) -> R,
) -> R {
    TRENDING_POSTS_COLLECTION.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_club_info<R>(f: impl FnOnce(&ClubInfoCell) -> R) -> R {
    CLUB_INFO.with(|s| f(&s.borrow()))
}

pub fn with_club_info_mut<R>(f: impl FnOnce(&mut ClubInfoCell) -> R) -> R {
    CLUB_INFO.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_chat_club_message_by_id<R>(f: impl FnOnce(&ChatClubMessageById) -> R) -> R {
    CHAT_CLUB_MESSAGE_BY_ID.with(|s| f(&s.borrow()))
}

pub fn with_chat_club_message_by_id_mut<R>(f: impl FnOnce(&mut ChatClubMessageById) -> R) -> R {
    CHAT_CLUB_MESSAGE_BY_ID.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_chat_club_messages_vec<R>(f: impl FnOnce(&ChatClubMessagesVec) -> R) -> R {
    CHAT_CLUB_MESSAGES.with(|s| f(&s.borrow()))
}

pub fn with_chat_club_messages_vec_mut<R>(f: impl FnOnce(&mut ChatClubMessagesVec) -> R) -> R {
    CHAT_CLUB_MESSAGES.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_canister_args<R>(f: impl FnOnce(&CanisterArgsCell) -> R) -> R {
    CANISTER_ARGS.with(|s| f(&s.borrow()))
}

pub fn with_canister_args_mut<R>(f: impl FnOnce(&mut CanisterArgsCell) -> R) -> R {
    CANISTER_ARGS.with(|s| f(&mut s.borrow_mut()))
}

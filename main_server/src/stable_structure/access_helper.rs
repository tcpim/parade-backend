use super::storages::*;

pub fn with_user_by_id<R>(f: impl FnOnce(&UserByIdMap) -> R) -> R {
    USER_BY_ID.with(|s| f(&s.borrow()))
}

pub fn with_user_by_id_mut<R>(f: impl FnOnce(&mut UserByIdMap) -> R) -> R {
    USER_BY_ID.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_user_names<R>(f: impl FnOnce(&UserNamesMap) -> R) -> R {
    USER_NAMES.with(|s| f(&s.borrow()))
}

pub fn with_user_names_mut<R>(f: impl FnOnce(&mut UserNamesMap) -> R) -> R {
    USER_NAMES.with(|s| f(&mut s.borrow_mut()))
}

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

pub fn with_street_posts_created<R>(f: impl FnOnce(&StreetPostsCreatedHeap) -> R) -> R {
    STREET_POSTS_CREATED.with(|s| f(&s.borrow()))
}

pub fn with_street_posts_created_mut<R>(f: impl FnOnce(&mut StreetPostsCreatedHeap) -> R) -> R {
    STREET_POSTS_CREATED.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_user_posts_created<R>(f: impl FnOnce(&UserPostsCreatedHeap) -> R) -> R {
    USER_POSTS_CREATED.with(|s| f(&s.borrow()))
}

pub fn with_user_posts_created_mut<R>(f: impl FnOnce(&mut UserPostsCreatedHeap) -> R) -> R {
    USER_POSTS_CREATED.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_collection_posts_created<R>(f: impl FnOnce(&CollectionPostsCreatedHeap) -> R) -> R {
    COLLECTION_POSTS_CREATED.with(|s| f(&s.borrow()))
}

pub fn with_collection_posts_created_mut<R>(
    f: impl FnOnce(&mut CollectionPostsCreatedHeap) -> R,
) -> R {
    COLLECTION_POSTS_CREATED.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_trending_posts_street<R>(f: impl FnOnce(&TrendingPostStreetHeap) -> R) -> R {
    TRENDING_POSTS_STREET.with(|s| f(&s.borrow()))
}

pub fn with_trending_posts_street_mut<R>(f: impl FnOnce(&mut TrendingPostStreetHeap) -> R) -> R {
    TRENDING_POSTS_STREET.with(|s| f(&mut s.borrow_mut()))
}

pub fn with_trending_posts_collection<R>(f: impl FnOnce(&TrendingPostCollectionHeap) -> R) -> R {
    TRENDING_POSTS_COLLECTION.with(|s| f(&s.borrow()))
}

pub fn with_trending_posts_collection_mut<R>(
    f: impl FnOnce(&mut TrendingPostCollectionHeap) -> R,
) -> R {
    TRENDING_POSTS_COLLECTION.with(|s| f(&mut s.borrow_mut()))
}

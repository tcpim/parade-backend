use crate::api::constants::DEFAULT_CHAT_PAGE_SIZE;
use crate::api_interface::chat_interface::{
    DeleteClubMessageRequest, GetClubMessagesRequest, GetClubMessagesResponse,
    ReactClubMessageRequest, SendClubMessageRequest, UpdateClubMessageRequest,
};
use candid::candid_method;
use ic_cdk_macros::{query, update};
use std::collections::BTreeMap;

use crate::api_interface::common_interface::{Cursor, ServerError};
use crate::models::chat_model::{ChatClubMessage, ChatClubMessageIdString};
use crate::stable_structure::access_helper::*;

#[update]
#[candid_method(update)]
pub fn send_club_message(request: SendClubMessageRequest) {
    let message = ChatClubMessage {
        id: ChatClubMessageIdString(request.message_id),
        user_id: request.sender,
        created_ts: request.created_ts,
        words: request.words,
        emoji_reactions: BTreeMap::new(),
        updated_ts: request.created_ts,
    };

    with_chat_club_message_by_id_mut(|map| {
        map.insert(message.id.clone(), message.clone());
    });

    with_chat_club_messages_vec_mut(|vec| {
        vec.push(&message.id.clone())
            .expect("Failed to add new club message");
    });
}

#[update]
#[candid_method(update)]
pub fn update_club_message(request: UpdateClubMessageRequest) -> Option<ServerError> {
    let mut error = None;
    with_chat_club_message_by_id_mut(|map| {
        if let Some(message) = map.get(&ChatClubMessageIdString(request.message_id.clone())) {
            if message.user_id != request.updater {
                error = Some(ServerError {
                    api_name: "update_club_message".to_string(),
                    error_message: "User is not the owner of this message".to_string(),
                })
            }
            let mut new_message = message;
            new_message.words = request.words;
            new_message.updated_ts = request.updated_ts;
            map.insert(
                ChatClubMessageIdString(request.message_id.clone()),
                new_message,
            );
        }
    });

    error
}

#[update]
#[candid_method(update)]
pub fn delete_club_message(request: DeleteClubMessageRequest) -> Option<ServerError> {
    with_chat_club_message_by_id_mut(|map| {
        if let Some(message) = map.get(&ChatClubMessageIdString(request.message_id.clone())) {
            if message.user_id != request.deleter {
                return Some(ServerError {
                    api_name: "delete_club_message".to_string(),
                    error_message: "User is not the owner of this message".to_string(),
                });
            }
            map.remove(&ChatClubMessageIdString(request.message_id.clone()));
        }

        None
    })
}

#[update]
#[candid_method(update)]
pub fn delete_all_club_message() {
    with_chat_club_message_by_id_mut(|storage| {
        let posts: Vec<_> = storage.range(..).collect();
        for (key, _value) in posts {
            storage.remove(&key);
        }
    });

    with_chat_club_messages_vec_mut(|storage| {
        for _ in 0..storage.len() {
            storage.pop().expect("Failed to delete message from vec");
        }
    });
}

#[update]
#[candid_method(update)]
pub fn react_club_message(request: ReactClubMessageRequest) {
    with_chat_club_message_by_id_mut(|map| {
        if let Some(message) = map.get(&ChatClubMessageIdString(request.message_id.clone())) {
            let mut emojis = message.emoji_reactions.clone();
            match emojis.get(&request.emoji) {
                Some(count) => {
                    emojis.insert(request.emoji.clone(), count + 1);
                }
                None => {
                    emojis.insert(request.emoji.clone(), 1);
                }
            }
            let mut new_message = message;
            new_message.emoji_reactions = emojis;

            map.insert(
                ChatClubMessageIdString(request.message_id.clone()),
                new_message,
            );
        }
    });
}

#[query]
#[candid_method(query)]
pub fn get_club_messages(request: GetClubMessagesRequest) -> GetClubMessagesResponse {
    let mut res: Vec<ChatClubMessageIdString> = vec![];
    with_chat_club_messages_vec(|vec| {
        let mut right = (vec.len() - 1) as i64;
        if request.cursor.0.is_some() {
            right = vec.len() as i64 - request.cursor.0.unwrap() as i64 - 1;
            if right < 0 {
                return GetClubMessagesResponse {
                    messages: vec![],
                    next_cursor: Cursor(None),
                    error: Some(ServerError {
                        api_name: "get_club_messages".to_string(),
                        error_message: "Cursor is out of range".to_string(),
                    }),
                };
            }
        }
        let mut left = right - DEFAULT_CHAT_PAGE_SIZE as i64 + 1;
        if request.limit.is_some() {
            left = right - request.limit.unwrap() as i64 + 1;
            if left < 0 {
                left = 0;
            }
        }

        // right = len - cursor= left

        for i in (left..=right).rev() {
            res.push(vec.get(i as u64).unwrap());
        }

        let mut next_cursor = Cursor(None);
        if left > 0 {
            next_cursor = Cursor(Some(vec.len() - left as u64));
        }

        let mut messages = vec![];
        with_chat_club_message_by_id_mut(|map| {
            for id in res {
                if let Some(message) = map.get(&id) {
                    messages.push(message.clone());
                }
            }
        });

        GetClubMessagesResponse {
            messages,
            next_cursor,
            error: None,
        }
    })
}

#[query]
#[candid_method(query)]
pub fn get_club_message_by_id(message_id: String) -> Option<ChatClubMessage> {
    with_chat_club_message_by_id(|map| map.get(&ChatClubMessageIdString(message_id)))
}

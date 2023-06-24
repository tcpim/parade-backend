use crate::api_interface::chat_interface::{
    GetClubMessagesRequest, ReactClubMessage, SendClubMessage, UpdateClubMessage,
};
use candid::candid_method;
use ic_cdk_macros::{query, update};
use std::collections::BTreeMap;

use crate::api_interface::club_interface::SetClubInfoRequest;
use crate::api_interface::common_interface::ServerError;
use crate::models::chat_model::{ChatClubMessage, ChatClubMessageIdString};
use crate::models::club_model::ClubInfo;
use crate::stable_structure::access_helper::*;

#[update]
#[candid_method(update)]
pub fn send_club_message(request: SendClubMessage) {
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
pub fn update_club_message(request: UpdateClubMessage) -> Option<ServerError> {
    let mut error = None;
    with_chat_club_message_by_id_mut(|map| {
        if let Some(message) = map.get(&ChatClubMessageIdString(request.message_id.clone())) {
            if message.user_id != request.updater {
                // TODO
            }
            let mut new_message = message.clone();
            new_message.words = request.words;
            new_message.updated_ts = request.updated_ts;
        }
    });

    error
}

#[update]
#[candid_method(update)]
pub fn delete_club_message(request: SendClubMessage) {}

#[update]
#[candid_method(update)]
pub fn react_club_message(request: ReactClubMessage) {}

#[update]
#[candid_method(update)]
pub fn delete_all_club_message(request: SendClubMessage) {}

#[query]
#[candid_method(query)]
pub fn get_club_messages(request: GetClubMessagesRequest) {}

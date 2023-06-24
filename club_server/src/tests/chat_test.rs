use crate::api::chat_api::{
    delete_club_message, get_club_message_by_id, get_club_messages, react_club_message,
    send_club_message, update_club_message,
};
use crate::api_interface::chat_interface::{
    DeleteClubMessageRequest, GetClubMessagesRequest, ReactClubMessageRequest,
    SendClubMessageRequest, UpdateClubMessageRequest,
};
use crate::api_interface::common_interface::Cursor;

#[test]
fn get_message_pagination_test() {
    // set up
    let request_1 = SendClubMessageRequest {
        message_id: "1".to_string(),
        sender: "tim".to_string(),
        created_ts: 1,
        words: "msg1".to_string(),
    };
    let request_2 = SendClubMessageRequest {
        message_id: "2".to_string(),
        sender: "tim".to_string(),
        created_ts: 2,
        words: "msg2".to_string(),
    };
    let request_3 = SendClubMessageRequest {
        message_id: "3".to_string(),
        sender: "tim".to_string(),
        created_ts: 3,
        words: "msg3".to_string(),
    };

    // act
    send_club_message(request_1);
    send_club_message(request_2);
    send_club_message(request_3);

    // assert
    // Get first page
    let response = get_club_messages(GetClubMessagesRequest {
        cursor: Cursor(None),
        limit: Some(2),
    });
    assert_eq!(response.messages.len(), 2);
    assert_eq!(response.next_cursor.0, Some(2));
    assert_eq!(response.messages[0].words, "msg3");
    assert_eq!(response.messages[1].words, "msg2");

    // Get second(last) page
    let response = get_club_messages(GetClubMessagesRequest {
        cursor: response.next_cursor,
        limit: Some(2),
    });
    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.next_cursor.0, None);
    assert_eq!(response.messages[0].words, "msg1");
}

#[test]
fn update_message_test() {
    // set up
    let request = SendClubMessageRequest {
        message_id: "1".to_string(),
        sender: "tim".to_string(),
        created_ts: 1,
        words: "msg1".to_string(),
    };
    send_club_message(request);

    // act
    let request = UpdateClubMessageRequest {
        message_id: "1".to_string(),
        words: "haha".to_string(),
        updater: "tim".to_string(),
        updated_ts: 1,
    };
    update_club_message(request);
    let response = get_club_message_by_id("1".to_string());

    // assert
    assert_eq!(response.unwrap().words, "haha");
}

#[test]
fn update_message_not_authorized_test() {
    // set up
    let request = SendClubMessageRequest {
        message_id: "1".to_string(),
        sender: "tim".to_string(),
        created_ts: 1,
        words: "msg1".to_string(),
    };
    send_club_message(request);

    // act
    let request = UpdateClubMessageRequest {
        message_id: "1".to_string(),
        words: "haha".to_string(),
        updater: "peter".to_string(),
        updated_ts: 1,
    };
    let response = update_club_message(request);

    // assert
    assert!(response
        .unwrap()
        .error_message
        .contains("User is not the owner of this message"));
}

#[test]
fn react_message_test() {
    // set up
    let request = SendClubMessageRequest {
        message_id: "1".to_string(),
        sender: "tim".to_string(),
        created_ts: 1,
        words: "msg1".to_string(),
    };
    send_club_message(request);

    // act
    let request = ReactClubMessageRequest {
        message_id: "1".to_string(),
        emoji: "aa".to_string(),
    };
    react_club_message(request);

    // assert
    let response = get_club_message_by_id("1".to_string());
    assert_eq!(response.clone().unwrap().emoji_reactions.len(), 1);
    assert_eq!(response.unwrap().emoji_reactions.get("aa"), Some(&1));
}

#[test]
fn delete_message_test() {
    // set up
    let request = SendClubMessageRequest {
        message_id: "1".to_string(),
        sender: "tim".to_string(),
        created_ts: 1,
        words: "msg1".to_string(),
    };
    send_club_message(request);

    // act
    delete_club_message(DeleteClubMessageRequest {
        message_id: "1".to_string(),
        deleter: "tim".to_string(),
        deleted_ts: 2,
    });

    // assert
    let response = get_club_message_by_id("1".to_string());
    assert!(response.is_none());
}

#[test]
fn delete_message_not_authorized_test() {
    // set up
    let request = SendClubMessageRequest {
        message_id: "1".to_string(),
        sender: "tim".to_string(),
        created_ts: 1,
        words: "msg1".to_string(),
    };
    send_club_message(request);

    // act
    let response = delete_club_message(DeleteClubMessageRequest {
        message_id: "1".to_string(),
        deleter: "peter".to_string(),
        deleted_ts: 2,
    });

    // assert
    assert!(response
        .unwrap()
        .error_message
        .contains("User is not the owner of this message"))
}

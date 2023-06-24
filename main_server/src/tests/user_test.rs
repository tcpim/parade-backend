use crate::api::user_api::{
    create_user, get_user_info, set_user_avatar, set_user_bio, set_user_name,
};
use crate::api_interface::user_interface::{
    SetUserAvatarRequest, SetUserBioRequest, SetUserNameRequest,
};
use crate::models::user_model::UserAvatar;

#[test]
fn create_user_test() {
    // act
    let user_id = "test_user_id".to_string();
    create_user(user_id.clone());

    // assert
    let response = get_user_info(user_id.clone());
    assert_eq!(response.user.unwrap().id, user_id);
}

#[test]
fn update_user_name_test() {
    // setup
    let user_id = "test_user_id".to_string();
    let name = "test_user_name".to_string();

    // act
    let request = SetUserNameRequest {
        user_id: user_id.clone(),
        new_name: name.clone(),
    };
    set_user_name(request);

    // assert
    let response = get_user_info(user_id.clone());
    assert_eq!(response.user.clone().unwrap().id, user_id);
    assert_eq!(response.user.clone().unwrap().user_name.unwrap(), name);

    // Test duplicate name
    let request = SetUserNameRequest {
        user_id: user_id.clone(),
        new_name: name.clone(),
    };
    let response = set_user_name(request);
    assert!(response.error.is_some());
    assert!(response
        .error
        .unwrap()
        .error_message
        .contains("already exists"));
}

#[test]
fn update_user_avatar_test() {
    // setup
    let user_id = "test_user_id".to_string();
    let avatar = vec![1, 2, 3];
    let mime_type = "image/png".to_string();

    // act
    let request = SetUserAvatarRequest {
        user_id: user_id.clone(),
        avatar: avatar.clone(),
        mime_type: mime_type.clone(),
    };

    set_user_avatar(request);

    // assert
    let response = get_user_info(user_id.clone());
    assert_eq!(response.user.clone().unwrap().id, user_id);
    assert_eq!(
        response.user.clone().unwrap().avatar.unwrap().mime_type,
        mime_type
    );
    assert_eq!(response.user.clone().unwrap().avatar.unwrap().data, avatar);

    // Test invalid mime type
    let request = SetUserAvatarRequest {
        user_id: user_id.clone(),
        avatar: avatar.clone(),
        mime_type: "image/gif".to_string(),
    };
    let response = set_user_avatar(request);
    assert!(response.error.is_some());
    assert!(response
        .error
        .unwrap()
        .error_message
        .contains("avatar mime type not supported"));
}

#[test]
fn update_user_bio_test() {
    // setup
    let user_id = "test_user_id".to_string();
    let bio = "test_user_bio".to_string();

    // act
    let request = SetUserBioRequest {
        user_id: user_id.clone(),
        bio: bio.clone(),
    };
    set_user_bio(request);

    // assert
    let response = get_user_info(user_id.clone());
    assert_eq!(response.user.clone().unwrap().id, user_id);
    assert_eq!(response.user.clone().unwrap().bio.unwrap(), bio);
}

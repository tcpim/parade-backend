use crate::api::user_api::{create_user, get_user_info, set_user_info};
use crate::api_interface::user_interface::SetUserInfoRequest;

#[test]
fn create_user_test() {
    // act
    let user_id = "test_user_id".to_string();
    create_user(user_id.clone());

    // assert
    let response = get_user_info(user_id.clone());
    assert_eq!(response.user.unwrap().pid, user_id);
}

#[test]
fn update_user_info() {
    // setup
    let user_id = "test_user_id".to_string();
    create_user(user_id.clone());
    let avatar: Vec<u8> = vec![1, 2, 3, 4, 5];
    let bio = "test_bio".to_string();

    // act
    let request = SetUserInfoRequest {
        user_id: user_id.clone(),
        user_name: Some("test_user_name".to_string()),
        user_avatar: Some(avatar.clone()),
        user_bio: Some(bio.clone()),
    };
    set_user_info(request);

    // assert
    let response = get_user_info(user_id.clone());
    assert_eq!(response.user.clone().unwrap().pid, user_id);
    assert_eq!(
        response.user.clone().unwrap().user_name.unwrap(),
        "test_user_name"
    );
    assert_eq!(response.user.clone().unwrap().avatar.unwrap(), avatar);
    assert_eq!(response.user.clone().unwrap().bio.unwrap(), bio);
}

#[test]
fn update_user_name_duplicate_error() {
    // set up
    let user_id_1 = "user_1".to_string();
    let user_id_2 = "user_2".to_string();
    create_user(user_id_1.clone());
    create_user(user_id_2.clone());

    let request = SetUserInfoRequest {
        user_id: user_id_1.clone(),
        user_name: Some("user_name_1".to_string()),
        user_avatar: None,
        user_bio: None,
    };
    set_user_info(request);

    // act
    let request = SetUserInfoRequest {
        user_id: user_id_2.clone(),
        user_name: Some("user_name_1".to_string()),
        user_avatar: None,
        user_bio: None,
    };
    let response = set_user_info(request);

    // assert
    println!("{:?}", response.user);
    assert!(response.error.is_some());
}

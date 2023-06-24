use crate::api_interface::common_interface::ServerError;
use crate::api_interface::user_interface::{
    GetUserInfoResponse, SetUserAvatarRequest, SetUserBioRequest, SetUserInfoResponse,
    SetUserNameRequest,
};
use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::stable_structure::access_helper::*;

use crate::models::user_model::{User, UserAvatar, UserNameStringKey, UserPrincipalStringKey};

// ######################
// APIs
// ######################

/**
Create a user with principal ID. If already exist then ignore
*/
#[update]
#[candid_method(update)]
pub fn create_user(user_id: String) {
    with_user_by_id_mut(|map| {
        match map.get(&UserPrincipalStringKey(user_id.clone())) {
            Some(_) => {
                // do nothing. user already created
            }
            None => {
                let user = User {
                    id: user_id.clone(),
                    user_name: None,
                    avatar: None,
                    bio: None,
                };
                map.insert(UserPrincipalStringKey(user_id), user);
            }
        }
    })
}

#[query]
#[candid_method(query)]
pub fn get_user_info(user_id: String) -> GetUserInfoResponse {
    with_user_by_id(|map| match map.get(&UserPrincipalStringKey(user_id)) {
        Some(user) => GetUserInfoResponse { user: Some(user) },
        None => GetUserInfoResponse { user: None },
    })
}

/**
Check avatar mime type and update avatar
*/
#[update]
#[candid_method(update)]
pub fn set_user_avatar(request: SetUserAvatarRequest) -> SetUserInfoResponse {
    let mime_type = request.mime_type.clone();
    if mime_type != "image/png" && mime_type != "image/jpeg" {
        return SetUserInfoResponse {
            user: User {
                id: request.user_id,
                user_name: None,
                avatar: None,
                bio: None,
            },
            error: Some(ServerError {
                api_name: "set_user_avatar".to_string(),
                error_message: "avatar mime type not supported".to_string(),
            }),
        };
    }

    let new_avatar = UserAvatar {
        data: request.avatar.clone(),
        mime_type: request.mime_type.clone(),
    };
    with_user_by_id_mut(|map| {
        match map.get(&UserPrincipalStringKey(request.user_id.clone())) {
            Some(existing_user) => {
                let mut new_user_info = existing_user;
                new_user_info.avatar = Some(new_avatar);
                map.insert(
                    UserPrincipalStringKey(request.user_id),
                    new_user_info.clone(),
                );

                SetUserInfoResponse {
                    user: new_user_info,
                    error: None,
                }
            }
            None => {
                // first time setup user info
                let user = User {
                    id: request.user_id.clone(),
                    user_name: None,
                    avatar: Some(new_avatar),
                    bio: None,
                };
                map.insert(UserPrincipalStringKey(request.user_id), user.clone());
                SetUserInfoResponse { user, error: None }
            }
        }
    })
}

/**
Check user name uniqueness and update user name
*/
#[update]
#[candid_method(update)]
pub fn set_user_name(request: SetUserNameRequest) -> SetUserInfoResponse {
    let mut error = None;
    let new_name = request.new_name.clone();
    if new_name.is_empty() {
        return SetUserInfoResponse {
            user: User {
                id: request.user_id,
                user_name: None,
                avatar: None,
                bio: None,
            },
            error: Some(ServerError {
                api_name: "set_user_name".to_string(),
                error_message: "user name cannot be empty".to_string(),
            }),
        };
    }

    with_user_by_id_mut(|user_map| {
        let user_opt = user_map.get(&UserPrincipalStringKey(request.user_id.clone()));
        let mut old_name = None;
        if user_opt.is_some() && user_opt.clone().unwrap().user_name.is_some() {
            old_name = user_opt.clone().unwrap().user_name;
        }

        // Insert new name and remove old name
        with_user_names_mut(|user_name_map| {
            if user_name_map.contains_key(&UserNameStringKey(new_name.clone())) {
                println!("user name already exists {:?}", new_name.clone());
                error = Some(ServerError {
                    api_name: "set_user_name".to_string(),
                    error_message: "user name already exists".to_string(),
                });
            } else {
                user_name_map.insert(UserNameStringKey(new_name.clone()), ());

                if old_name.is_some() {
                    user_name_map.remove(&UserNameStringKey(old_name.clone().unwrap()));
                }
            }
        });

        if error.is_some() {
            return SetUserInfoResponse {
                user: User {
                    id: request.user_id.clone(),
                    user_name: None,
                    avatar: None,
                    bio: None,
                },
                error,
            };
        }

        // Update user name in user info
        let mut new_user_info = user_opt.unwrap_or(User {
            id: request.user_id.clone(),
            user_name: None,
            avatar: None,
            bio: None,
        });
        new_user_info.user_name = Some(new_name.clone());

        user_map.insert(
            UserPrincipalStringKey(request.user_id.clone()),
            new_user_info.clone(),
        );

        SetUserInfoResponse {
            user: new_user_info,
            error: None,
        }
    })
}

#[update]
#[candid_method(update)]
pub fn set_user_bio(request: SetUserBioRequest) -> SetUserInfoResponse {
    with_user_by_id_mut(|map| {
        match map.get(&UserPrincipalStringKey(request.user_id.clone())) {
            Some(existing_user) => {
                let mut new_user_info = existing_user;
                new_user_info.bio = Some(request.bio.clone());
                map.insert(
                    UserPrincipalStringKey(request.user_id),
                    new_user_info.clone(),
                );

                SetUserInfoResponse {
                    user: new_user_info,
                    error: None,
                }
            }
            None => {
                // first time setup user info
                let user = User {
                    id: request.user_id.clone(),
                    user_name: None,
                    avatar: None,
                    bio: Some(request.bio.clone()),
                };
                map.insert(UserPrincipalStringKey(request.user_id), user.clone());

                SetUserInfoResponse { user, error: None }
            }
        }
    })
}

#[update]
#[candid_method(update)]
pub fn delete_all_users() {
    // You might run into candid deserialization error if you changed the proto
    // Solution: you can temporarily modify the proto to fit the earlier version and make the deserialization work
    // and then change it back after deletion
    with_user_names_mut(|storage| {
        let posts: Vec<_> = storage.range(..).collect();
        for (key, _value) in posts {
            storage.remove(&key);
        }
    });

    with_user_by_id_mut(|storage| {
        let posts: Vec<_> = storage.range(..).collect();
        for (key, _value) in posts {
            storage.remove(&key);
        }
    });
}

use crate::api_interface::common_interface::ServerError::SetUserInfoError;
use crate::api_interface::user_interface::{
    GetUserInfoResponse, SetUserInfoRequest, SetUserInfoResponse,
};
use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::stable_structure::access_helper::*;

use crate::models::user_model::{User, UserNameStringKey, UserPrincipalStringKey};

// ######################
// APIs
// ######################

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
                    pid: user_id.clone(),
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
        Some(user) => GetUserInfoResponse {
            user: Some(user.clone()),
        },
        None => GetUserInfoResponse { user: None },
    })
}

/**
1. Check user name uniqueness
2. Update user info
*/
#[update]
#[candid_method(update)]
pub fn set_user_info(request: SetUserInfoRequest) -> SetUserInfoResponse {
    let mut error = None;
    if request.user_name.is_some() {
        with_user_names_mut(|map| {
            if map.contains_key(&UserNameStringKey(request.user_name.clone().unwrap())) {
                println!("user name already exists {:?}", request.user_name);
                error = Some(SetUserInfoError("user name already exists".to_string()));
            } else {
                map.insert(UserNameStringKey(request.user_name.clone().unwrap()), ());
            }
        })
    }
    if error.is_some() {
        return SetUserInfoResponse {
            user: User {
                pid: request.user_id.clone(),
                user_name: None,
                avatar: None,
                bio: None,
            },
            error,
        };
    }

    with_user_by_id_mut(|map| {
        let user_opt = map.get(&UserPrincipalStringKey(request.user_id.clone()));
        if user_opt.is_none() {
            let user = User {
                pid: request.user_id.clone(),
                user_name: request.user_name,
                avatar: request.user_avatar,
                bio: request.user_bio,
            };

            map.insert(UserPrincipalStringKey(request.user_id), user.clone());
            SetUserInfoResponse { user, error: None }
        } else {
            let mut user = user_opt.unwrap().clone();
            if request.user_name.is_some() {
                user.user_name = request.user_name;
            }
            if request.user_avatar.is_some() {
                user.avatar = request.user_avatar;
            }
            if request.user_bio.is_some() {
                user.bio = request.user_bio;
            }
            map.insert(UserPrincipalStringKey(request.user_id), user.clone());

            SetUserInfoResponse { user, error: None }
        }
    })
}

use crate::api::helpers_api::is_caller_authorized;
use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::api_interface::club_interface::SetClubInfoRequest;
use crate::api_interface::common_interface::ServerError;
use crate::models::club_model::ClubInfo;
use crate::stable_structure::access_helper::*;

#[update]
#[candid_method(update)]
pub fn set_club_info(request: SetClubInfoRequest) -> Option<ServerError> {
    if !is_caller_authorized() {
        return Some(ServerError {
            api_name: "set_club_info".to_string(),
            error_message: format!("Unauthorized caller: {}", ic_cdk::caller().to_string()),
        });
    }
    with_club_info_mut(|cell| {
        cell.set(request.info).expect("Failed to set club info");
    });

    None
}

#[query]
#[candid_method(query)]
pub fn get_club_info() -> ClubInfo {
    with_club_info(|cell| cell.get().clone())
}

use candid::candid_method;
use ic_cdk_macros::{query, update};

use crate::api_interface::club::SetClubInfoRequest;
use crate::models::club::ClubInfo;
use crate::stable_structure::access_helper::*;

#[update]
#[candid_method(update)]
pub fn set_club_info(request: SetClubInfoRequest) {
    with_club_info_mut(|cell| {
        cell.set(request.info).expect("Failed to set club info");
    })
}

#[query]
#[candid_method(query)]
pub fn get_club_info() -> ClubInfo {
    with_club_info(|cell| cell.get().clone())
}

use crate::api_interface::init_interface::InitClubCanisterRequest;
use crate::models::init_model::CanisterArgs;
use crate::stable_structure::access_helper::*;
use candid::candid_method;
use ic_cdk_macros::init;
use ic_cdk_macros::query;

// ######################
// APIs
// ######################

#[init]
#[candid_method(init)]
fn canister_init(args: InitClubCanisterRequest) {
    with_club_info_mut(|cell| {
        cell.set(args.info.clone())
            .expect("Failed to set club info");
    });

    with_canister_args_mut(|cell| {
        cell.set(CanisterArgs {
            env: args.env.clone(),
        })
        .expect("Failed to set canister args");
    });
}

#[query]
#[candid_method(query)]
pub fn get_canister_args() -> String {
    with_canister_args(|cell| cell.get().env.clone())
}

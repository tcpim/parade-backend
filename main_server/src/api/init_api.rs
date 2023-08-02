use crate::api_interface::init_interface::InitCanisterRequest;
use candid::candid_method;
use ic_cdk_macros::init;

use crate::models::init_model::CanisterArgs;
use crate::stable_structure::access_helper::*;

// ######################
// APIs
// ######################

#[init]
#[candid_method(init)]
fn canister_init(args: InitCanisterRequest) {
    with_canister_args_mut(|cell| {
        cell.set(CanisterArgs {
            env: args.env.clone(),
        })
        .expect("Failed to set canister args");
    });
}

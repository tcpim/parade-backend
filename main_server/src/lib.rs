mod api;
mod api_interface;
mod generate_candid;
mod models;
mod stable_structure;
mod tests;

// ######################
// Below code is to generate candid file
// ######################
// Don't remove following imports!! Needed by candid::export_service

use crate::api_interface::post_reaction::*;
use crate::api_interface::post_reply::*;
use crate::api_interface::post_trending::*;
use crate::api_interface::posts::*;
candid::export_service!();

pub fn export_candid() -> String {
    __export_service()
}

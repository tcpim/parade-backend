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
use crate::api_interface::chat_interface::*;
use crate::api_interface::club_interface::*;
use crate::api_interface::common_interface::*;
use crate::api_interface::post_reaction_interface::*;
use crate::api_interface::post_reply_interface::*;
use crate::api_interface::post_trending_interface::*;
use crate::api_interface::posts_interface::*;
use crate::models::club_model::*;
candid::export_service!();

pub fn export_candid() -> String {
    __export_service()
}

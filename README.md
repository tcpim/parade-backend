## Upgrade dependencies
cargo update --package 

## To generate candid file
- comment out .cargo/config.toml file
- run `cargo test`

## For each api, need to add macro
- #[update] / #[query] to specify whether it is a query / update call
- #[candid_method(update)] / query to let test method generate candid file

## Local Deployment
### When deploying locally from the start (empty state)
 - `dfx deploy main_server` and mark the canister ID
 - Change the canister ID in /club_server/src/api/constants.rs
 - Run 
   - `dfx deploy ludo_arts_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ludo-arts" }})'
     dfx deploy motoko_ghost_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "motoko-ghost" }})'
     dfx deploy poked_bots_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "poked-bots" }})'`

### Update candid types 
- run `dfx generate` and paste files to frontend
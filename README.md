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

- give permission to sh files

```
chmod +x start_main_server.sh
chmod +x start_club_server.sh
```

- run `./start_main_server.sh` to start main server. Note down the main server canister ID after finish and change it
  in parade-backend/club_server/src/api/constants.rs
- run `./start_club_server.sh` to start club server.
- Sometimes you need to run in reinstall mode to after you modify candid interface, run with -mode=reinstall

```
./start_main_server.sh -mode=reinstall
./start_club_server.sh -mode=reinstall
```

## Deploy to prod

```
  dfx deploy main_server --argument='(record {env = "prod"})' --network=ic
  dfx deploy ludo_arts_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ludo-arts"}; env="prod"})'  --network=ic
  dfx deploy motoko_ghost_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "motoko-ghost"}; env = "prod"})'  --network=ic
  dfx deploy poked_bots_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "poked-bots"}; env = "prod"})'   --network=ic
```

### Update candid types

- run `dfx generate` and paste files to frontend

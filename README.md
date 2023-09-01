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

./start_main_server_prod.sh
./start_club_server_prod.sh

```

### Update candid types

- run `dfx generate` and paste files to frontend

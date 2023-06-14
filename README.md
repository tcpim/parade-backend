### Upgrade dependencies
cargo update --package 

### To generate candid file
- comment out .cargo/config.toml file
- run `cargo test`

### For each api, need to add macro
- #[update] / #[query] to specify whether it is a query / update call
- #[candid_method(update)] / query to let test method generate candid file
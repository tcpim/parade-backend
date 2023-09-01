# Parse named arguments using a while loop
while [[ "$#" -gt 0 ]]; do
  case "$1" in
    -mode=*)
      # Remove the '-mode=' prefix to get the value of the 'mode' argument
      mode="${1#*=}"
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
  shift
done

# Check the value of 'mode'
if [[ "$mode" = "reinstall" ]]; then
  dfx deploy ludo_arts_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ludo-arts"}; env="prod"})' -m=reinstall --network=ic
  dfx deploy poked_bots_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "poked-bots"}; env = "prod"})' -m=reinstall --network=ic
  dfx deploy motoko_ghost_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "motoko-ghost"}; env = "prod"})' -m=reinstall --network=ic
  dfx deploy boxy-dude_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "boxy-dude"}; env = "prod"})' -m=reinstall --network=ic
  dfx deploy dscvr-airdrop_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "dscvr-airdrop"}; env = "prod"})' -m=reinstall --network=ic
  dfx deploy cubetopia_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "cubetopia"}; env = "prod"})' -m=reinstall --network=ic
  dfx deploy ic-punks_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ic-punks"}; env = "prod"})' -m=reinstall --network=ic
else
  dfx deploy ludo_arts_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ludo-arts"}; env="prod"})' --network=ic
  dfx deploy poked_bots_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "poked-bots"}; env = "prod"})' --network=ic
  dfx deploy motoko_ghost_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "motoko-ghost"}; env = "prod"})' --network=ic
  dfx deploy boxy-dude_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "boxy-dude"}; env = "prod"})' --network=ic
  dfx deploy dscvr-airdrop_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "dscvr-airdrop"}; env = "prod"})' --network=ic
  dfx deploy cubetopia_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "cubetopia"}; env = "prod"})' --network=ic
  dfx deploy ic-punks_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ic-punks"}; env = "prod"})' --network=ic
fi


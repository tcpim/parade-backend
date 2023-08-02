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
  dfx deploy ludo_arts_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ludo-arts"}; env="dev"})' -m=reinstall
  dfx deploy motoko_ghost_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "motoko-ghost"}; env = "dev"})' -m=reinstall
  dfx deploy poked_bots_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "poked-bots"}; env = "dev"})' -m=reinstall
else
  dfx deploy ludo_arts_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "ludo-arts"}; env="dev"})' 
  dfx deploy motoko_ghost_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "motoko-ghost"}; env = "dev"})' 
  dfx deploy poked_bots_club --argument='(record {info = record { club_description = ""; club_name = ""; club_id = "poked-bots"}; env = "dev"})' 
fi


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
  dfx deploy main_server --argument='(record {env = "dev"})' -m=reinstall
else
  dfx deploy main_server --argument='(record {env = "dev"})'
fi

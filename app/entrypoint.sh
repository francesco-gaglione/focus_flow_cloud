#!/bin/sh

# Find config.json location (handle Flutter web asset structure variations)
CONFIG_FILE=$(find /usr/share/nginx/html -name "config.json" | head -n 1)

if [ -z "$CONFIG_FILE" ]; then
  echo "Error: config.json not found in /usr/share/nginx/html"
else
  echo "Found config file at $CONFIG_FILE"

  # Use environment variables if set, otherwise default to localhost (or keep existing)
  # We construct the JSON manually to avoid sed issues and ensure clean state

  curr_base_url=${BASE_URL:-"http://localhost:8080"}
  curr_ws_url=${WS_URL:-"ws://localhost:8080/ws/workspace/session"}

  echo "Applying configuration:"
  echo "BASE_URL: $curr_base_url"
  echo "WS_URL: $curr_ws_url"

  # Overwrite the file with new JSON
  echo "{\"BASE_URL\": \"$curr_base_url\", \"WS_URL\": \"$curr_ws_url\"}" > "$CONFIG_FILE"

  echo "Configuration file content:"
  cat "$CONFIG_FILE"
fi

# Execute the passed command (nginx)
exec "$@"

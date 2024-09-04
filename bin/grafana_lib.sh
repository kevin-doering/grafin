#!/bin/bash

function dotenv() {
  # Source the .env file to load environment variables
  DOT_ENV_PATH="../.env"
  if [ -f $DOT_ENV_PATH ]; then
    # shellcheck disable=SC2046
    export $(grep -v '^#' $DOT_ENV_PATH | xargs)
  else
    echo "Error: $DOT_ENV_PATH file not found."
    exit 1
  fi
}

# Function to check if a variable is empty
function check_if_empty() {
  local var_value="$1"
  local var_name="$2"
  if [ -z "$var_value" ]; then
    echo "Error: $var_name is missing."
    exit 1
  fi
}

# Function to validate datetime format
function validate_datetime_format() {
  local datetime="$1"
  local var_name="$2"
  if ! date -d "$datetime" "+%Y-%m-%d %H:%M" >/dev/null 2>&1; then
    echo "Error: $var_name is not in the correct format [format: %Y-%m-%d %H:%M]."
    exit 1
  fi
}

# Function to convert datetime to epoch milliseconds
function datetime_to_epoch_ms() {
  local datetime="$1"
  time=$(date -d "$datetime" +%s%3N)
  echo "$time"
}

# Function to create an annotation in Grafana
function create_grafana_annotation() {
  local dashboardUid="$1"
  local panelId="$2"
  local startDatetime="$3"
  local endDatetime="$4"
  local tags="$5"
  local text="$6"
  local grafana_url="$7"
  local api_key="$8"
  local time
  local timeEnd

  # Transform datetime format
  time=$(datetime_to_epoch_ms "$startDatetime")
  timeEnd=$(datetime_to_epoch_ms "$endDatetime")

  # Create the JSON payload
  json_payload=$(jq -n \
    --arg dashboardUID "$dashboardUid" \
    --argjson panelId "$panelId" \
    --argjson time "$time" \
    --argjson timeEnd "$timeEnd" \
    --arg tags "$tags" \
    --arg text "$text" \
    '{
        "dashboardUID": $dashboardUID,
        "panelId": $panelId,
        "time": $time,
        "timeEnd": $timeEnd,
        "tags": $tags | split(","),
        "text": $text
      }')

  echo "$json_payload"

  # Send the POST request to Grafana API
  response=$(curl -X POST -s "$grafana_url"annotations \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $api_key" \
    -d "$json_payload")

  echo "$response"
}

#!/bin/bash

# Function to convert datetime to epoch milliseconds
datetime_to_epoch_ms() {
  local datetime="$1"
  time=$(date -d "$datetime" +%s%3N)
  echo "$time"
}

# Function to create an annotation in Grafana
create_grafana_annotation() {
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

  echo "$grafana_url"

  # Transform datetime format
  time=$(datetime_to_epoch_ms "$startDatetime")
  timeEnd=$(datetime_to_epoch_ms "$endDatetime")

  echo "$time"
  echo "$timeEnd"

  # Create the JSON payload
  json_payload=$(jq -n \
    --arg dashboardUID "$dashboardUid" \
    --arg panelId "$panelId" \
    --arg time "$time" \
    --arg timeEnd "$timeEnd" \
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

  # Send the POST request to Grafana API
  response=$(curl -X POST -v "$grafana_url"annotations \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $api_key" \
    -d "$json_payload")

  echo "$response"
}

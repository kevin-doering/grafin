#!/bin/bash

SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_PATH}" || exit

source ./grafana_lib.sh

dotenv

teamIds=".created_teams.txt"
folderUIDs=".created_folders.txt"

function delete_resource {
  local resource="$1"
  local id="$2"
  response=$(
    curl -X DELETE "$GRAFANA_API_PATH/$resource/$id" \
      -H "Accept: application/json" \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $SERVICE_ACCOUNT_TOKEN"
  )
  message=$(echo "$response" | jq '.message')
  echo "$message"
}

if [[ -f "$teamIds" ]]; then
  while IFS= read -r teamId; do
    msg=$(delete_resource "teams" "$teamId")
    echo "$msg : $teamId"
  done <"$teamIds"
else
  echo "File with $teamIds does not exist."
fi

if [[ -f "$folderUIDs" ]]; then
  while IFS= read -r folderUid; do
    msg=$(delete_resource "folders" "$folderUid")
    echo "$msg : $folderUid"
  done <"$folderUIDs"
else
  echo "File with $folderUIDs does not exist."
fi

time=$(date +"%Y-%m-%d %H:%M:%S")

mv .created_teams.txt ".deleted_teams_$time.txt"
mv .created_folders.txt ".deleted_folders_$time.txt"

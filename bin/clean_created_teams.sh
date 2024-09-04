#!/bin/bash

API="https://etaps.grafana.intern/api"
TOKEN="<grafana_service_account_token>"
teamIds=".created_teams.txt"
folderUids=".created_folders.txt"

function delete_resource {
  local resource="$1"
  local id="$2"
  response=$(
    curl -s -X DELETE "$API/$resource/$id" \
      -H "Accept: application/json" \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $TOKEN"
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

if [[ -f "$folderUids" ]]; then
  while IFS= read -r folderUid; do
    msg=$(delete_resource "folders" "$folderUid")
    echo "$msg : $folderUid"
  done <"$folderUids"
else
  echo "File with $folderUids does not exist."
fi

time=$(date +"%Y-%m-%d %H:%M:%S")

mv .created_teams.txt ".deleted_teams_$time.txt"
mv .created_folders.txt ".deleted_folders_$time.txt"

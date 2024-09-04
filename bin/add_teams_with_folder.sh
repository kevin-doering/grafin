#!/bin/bash

team="$1"
orgId="${2:1}"
folder="$team"
admin="$team"
viewer="$team-Viewer"

API="https://etaps.grafana.intern/api"
TOKEN="<grafana_service_account_token>"

function is_team_name_taken {
  local name="$1"
  team=$(check_team_exists "$name")
  count=$(echo "$team" | jq '.totalCount')
  # team_count is a valid number and greater than 0
  if [[ "$count" =~ ^[0-9]+$ ]] && [[ "$count" -gt 0 ]]; then
    echo "The team name: $name is already taken. Try another one! Exiting.." && exit
  else
    echo "Generating team with name: $name"
  fi
}

function check_team_exists {
  local query_name="$1"
  response=$(curl -s -X GET "$API/teams/search?query=$query_name" \
    -H "Accept: application/json" \
    -H "Authorization: Bearer $TOKEN")
  echo "$response"
}

function add_team {
  local team="$1"
  local org_id="${2:1}"
  response=$(curl -s -X POST $API/teams \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d '{
        "name": "'"$team"'",
        "orgId": "'"$org_id"'"
    }')
  echo "$response"
}

function add_team_folder {
  local folder="$1"
  response=$(curl -s -X POST "$API/folders" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d '{
        "title": "'"$folder"'"
    }')
  echo "$response"
}

function set_team_folder_permission {
  local folder_uid=$(echo "$1" | tr -d '"')
  local admin_team_id="$2"
  local viewer_team_id="$3"
  response=$(curl -s -X POST "$API/folders/$folder_uid/permissions" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d '{
        "items": [
          {
            "role": "Viewer",
            "permission": 1
          },
          {
            "role": "Editor",
            "permission": 2
          },
          {
            "role": "Admin",
            "permission": 4
          },
          {
            "teamId": '"$admin_team_id"',
            "permission": 4
          },
          {
            "teamId": '"$viewer_team_id"',
            "permission": 1
          }
        ]
      }')
  echo "$response"
}

isAdminTeamNameTaken=$(is_team_name_taken "$admin")
isViewerTeamNameTaken=$(is_team_name_taken "$viewer")
echo "$isAdminTeamNameTaken"
echo "$isViewerTeamNameTaken"

addAdminTeamResponse=$(add_team "$admin" "$orgId")
addViewerTeamResponse=$(add_team "$viewer" "$orgId")
adminTeamMessage=$(echo "$addAdminTeamResponse" | jq '.message')
adminTeamId=$(echo "$addAdminTeamResponse" | jq '.teamId')
viewerTeamMessage=$(echo "$addViewerTeamResponse" | jq '.message')
viewerTeamId=$(echo "$addViewerTeamResponse" | jq '.teamId')
echo "$adminTeamMessage [id: $adminTeamId, name: $admin]"
echo "$viewerTeamMessage [id: $viewerTeamId, name: $viewer]"

addFolderResponse=$(add_team_folder "$folder")
folderUid=$(echo "$addFolderResponse" | jq '.uid')
folderTitle=$(echo "$addFolderResponse" | jq '.title')
echo "Folder created [uid: $folderUid, title: $folderTitle]"

setPermissionResponse=$(set_team_folder_permission "$folderUid" "$adminTeamId" "$viewerTeamId")
setPermissionMessage=$(echo "$setPermissionResponse" | jq '.message')
echo "$setPermissionMessage"

echo "$adminTeamId" >>.created_teams.txt
echo "$viewerTeamId" >>.created_teams.txt

echo "$folderUid" | tr -d '"' >>.created_folders.txt

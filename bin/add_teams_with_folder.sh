#!/bin/bash

SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_PATH}" || exit

source ./grafana_lib.sh

dotenv

team="$1"
orgId="${2:1}"
folder="$team"
admin="$team"
viewer="$team-Viewer"

function is_team_name_taken {
  local name="$1"
  team=$(check_team_exists "$name")
  count=$(echo "$team" | jq '.totalCount')
  # team_count is a valid number and greater than 0
  if [[ "$count" =~ ^[0-9]+$ ]] && [[ "$count" -gt 0 ]]; then
    echo "y"
  else
    echo "n"
    echo "Generating team with name: $name"
  fi
}

function check_team_exists {
  local query_name="$1"
  response=$(curl "$GRAFANA_API_PATH/teams/search?query=$query_name" \
    -H "Accept: application/json" \
    -H "Authorization: Bearer $SERVICE_ACCOUNT_TOKEN")
  echo "$response"
}

function add_team {
  local team="$1"
  local org_id="${2:1}"
  response=$(curl -X POST "${GRAFANA_API_PATH}/teams" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $SERVICE_ACCOUNT_TOKEN" \
    -d '{
        "name": "'"$team"'",
        "orgId": "'"$org_id"'"
    }')
  echo "$response"
}

function add_team_folder {
  local folder="$1"
  response=$(curl -X POST "${GRAFANA_API_PATH}/folders" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $SERVICE_ACCOUNT_TOKEN" \
    -d '{
        "title": "'"$folder"'"
    }')
  echo "$response"
}

function set_team_folder_permission {
  local folder_uid="$1"
  local admin_team_id="$2"
  local viewer_team_id="$3"

  folder_uid=$(echo "$folder_uid" | tr -d '"')
  response=$(curl -X POST "${GRAFANA_API_PATH}/folders/$folder_uid/permissions" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $SERVICE_ACCOUNT_TOKEN" \
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

if [ "$isAdminTeamNameTaken" == "y" ]; then
  echo "The team name: $admin is already taken. Try another one! Exiting.."
  exit
fi

if [ "$isViewerTeamNameTaken" == "y" ]; then
  echo "The team name: $viewer is already taken. Try another one! Exiting.."
  exit
fi

addAdminTeamResponse=$(add_team "$admin" "$orgId")
addViewerTeamResponse=$(add_team "$viewer" "$orgId")
adminTeamMessage=$(echo "$addAdminTeamResponse" | jq '.message')
adminTeamId=$(echo "$addAdminTeamResponse" | jq '.teamId')
viewerTeamMessage=$(echo "$addViewerTeamResponse" | jq '.message')
viewerTeamId=$(echo "$addViewerTeamResponse" | jq '.teamId')
echo "$adminTeamMessage [id: $adminTeamId, name: $admin]"
echo "$viewerTeamMessage [id: $viewerTeamId, name: $viewer]"

addFolderResponse=$(add_team_folder "$folder")
echo "$addFolderResponse"
folderUid=$(echo "$addFolderResponse" | jq '.uid')
folderTitle=$(echo "$addFolderResponse" | jq '.title')
echo "Folder: [uid: $folderUid, title: $folderTitle]"

setPermissionResponse=$(set_team_folder_permission "$folderUid" "$adminTeamId" "$viewerTeamId")
setPermissionMessage=$(echo "$setPermissionResponse" | jq '.message')
echo "$setPermissionMessage"

echo "$adminTeamId" >>.created_teams.txt
echo "$viewerTeamId" >>.created_teams.txt

echo "$folderUid" | tr -d '"' >>.created_folders.txt

#!/usr/bin/env bash

# @author Kevin Doering <k.doering.dt@gmail.com>
# @description Adds annotations to all panels within the named dashboard/folder scope by sending multiple requests to the grafana server utilizing cURL and jq.
# @usage ./bin/add_anno_to_panels_in_scope.sh folderName dashboardName "2024-09-04 08:00" "2024-09-04 08:30" tag comment

# get absolute path to script and change context to script folder
SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_PATH}" || exit

# Source the functional library
source ./grafana_lib.sh

dotenv

# Variables passed as arguments
folderName="$1"
dashboardName="$2"
startDatetime="$3"
endDatetime="$4"
tags="$5"
comment="$6"

# Check env vars
check_if_empty "$SERVICE_ACCOUNT_TOKEN" "SERVICE_ACCOUNT_TOKEN"
check_if_empty "$GRAFANA_API_PATH" "GRAFANA_API_PATH"

# Check each variable passed as argument
check_if_empty "$folderName" "folderName"
check_if_empty "$dashboardName" "dashboardName"
check_if_empty "$startDatetime" "startDatetime"
check_if_empty "$endDatetime" "endDatetime"
check_if_empty "$tags" "tags"
check_if_empty "$comment" "comment"

# Validate startDatetime and endDatetime formats
validate_datetime_format "$startDatetime" "startDatetime"
validate_datetime_format "$endDatetime" "endDatetime"

echo "Required variables are set!"
echo "Datetime values are correctly formatted: [format: %Y-%m-%d %H:%M]"
echo "Sending annotation requests to the grafana server."
delimiter

# Create the annotation using the sourced functional library
response=$(add_annotations_to_all_panel_within_the_specified_dash_type_scope "$folderName" "$dashboardName" "$startDatetime" "$endDatetime" "$tags" "$comment" "$GRAFANA_API_PATH" "$SERVICE_ACCOUNT_TOKEN")

echo "$response"

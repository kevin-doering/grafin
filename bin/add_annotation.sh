#!/usr/bin/env bash

# @author Kevin Doering <k.doering.dt@gmail.com>
# @description creates annotations within the grafana server database utilizing the grafana http api with cURL
# @usage ./bin/add_annotation.sh bdvea4glj4fswf 1 "2024-09-04 08:00" "2024-09-04 08:30" tag comment

# get absolute path to script and change context to script folder
SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_PATH}" || exit

# Source the functional library
source ./grafana_lib.sh

dotenv

# Variables passed as arguments
dashboardUid="$1"
panelId="$2"
startDatetime="$3"
endDatetime="$4"
tags="$5"
comment="$6"

# Check env vars
check_if_empty "$SERVICE_ACCOUNT_TOKEN" "SERVICE_ACCOUNT_TOKEN"
check_if_empty "$GRAFANA_API_PATH" "GRAFANA_API_PATH"

# Check each variable passed as argument
check_if_empty "$dashboardUid" "dashboardUid"
check_if_empty "$panelId" "panelId"
check_if_empty "$startDatetime" "startDatetime"
check_if_empty "$endDatetime" "endDatetime"
check_if_empty "$tags" "tags"
check_if_empty "$comment" "comment"

# Validate startDatetime and endDatetime formats
validate_datetime_format "$startDatetime" "startDatetime"
validate_datetime_format "$endDatetime" "endDatetime"

echo "Required variables are set!"
echo "Datetime values are correctly formatted: [format: %Y-%m-%d %H:%M]"
echo "Sending an annotation request to the grafana server."

# Create the annotation using the function from the library
response=$(create_grafana_annotation "$dashboardUid" "$panelId" "$startDatetime" "$endDatetime" "$tags" "$comment" "$GRAFANA_API_PATH" "$SERVICE_ACCOUNT_TOKEN")

echo "$response"

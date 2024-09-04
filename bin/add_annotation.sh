#!/usr/bin/env bash

# @author Kevin Doering <k.doering.dt@gmail.com>
# @description creates annotations within the grafana server database utilizing the grafana http api with cURL
# @usage ./bin/add_annotation.sh bdvea4glj4fswf 1 "2024-09-04 08:00" "2024-09-04 08:30" tag comment

# get absolute path to script and change context to script folder
SCRIPT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_PATH}" || exit

# Source the functional library
source ./grafana_lib.sh

# Variables passed as arguments
dashboardUid="$1"
panelId="$2"
startDatetime="$3"
endDatetime="$4"
tags="$5"
comment="$6"

# Source the .env file to load environment variables
DOT_ENV_PATH="../.env"
if [ -f $DOT_ENV_PATH ]; then
  # shellcheck disable=SC2046
  export $(grep -v '^#' $DOT_ENV_PATH | xargs)
else
  echo "Error: $DOT_ENV_PATH file not found."
  exit 1
fi

# Function to check if a variable is empty
check_if_empty() {
  local var_value="$1"
  local var_name="$2"
  if [ -z "$var_value" ]; then
    echo "Error: $var_name is missing."
    exit 1
  fi
}

# Function to validate datetime format
validate_datetime_format() {
  local datetime="$1"
  local var_name="$2"
  if ! date -d "$datetime" "+%Y-%m-%d %H:%M" >/dev/null 2>&1; then
    echo "Error: $var_name is not in the correct format [format: %Y-%m-%d %H:%M]."
    exit 1
  fi
}

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

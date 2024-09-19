#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum GrafanaCliError {
    /// an I/O error occurred: {0}
    IO(#[from] std::io::Error),

    /// a network error occurred: {0}
    Request(#[from] reqwest::Error),

    /// no team id received from grafana server on team creation
    NoTeamIdReceivedFromGrafanaOnTeamCreation,

    /// can not update the permissions on a non-existing folder
    CanNotUpdatePermissionsOnNonExistingFolder,

    /// datetime parse error: {0}
    CanNotParseDateTimeToEpochTimeMillis(#[from] chrono::ParseError),

    /// can not parse the start_datetime to epoch time millis [format: %Y-%m-%d %H:%M]
    CanNotParseTheStartDateTimeToEpochTimeMillis,

    /// can not parse the end_datetime to epoch time millis [format: %Y-%m-%d %H:%M]
    CanNotParseTheEndDateTimeToEpochTimeMillis,

    /// an error occurred during request body parsing: {0}
    InvalidResponseFormat(String),

    /// can not add a folder without a title
    CanNotAddFolderWithoutTitle,

    /// can not encode url params to string: {0}
    CanNotEncodeUrlParamToString(#[from] serde_url_params::Error),
}
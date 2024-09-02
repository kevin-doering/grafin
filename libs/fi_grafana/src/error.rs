#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum GrafanaCliError {
    /// a network error occurred: {0}
    Request(#[from] reqwest::Error),

    /// an I/O error occurred: {0}
    IO(#[from] std::io::Error),

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
}
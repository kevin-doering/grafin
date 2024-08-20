#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum GrafanaCliError {
    /// A network error occurred: {0}
    Request(#[from] reqwest::Error),

    /// An I/O error occurred: {0}
    IO(#[from] std::io::Error),

    /// Team module error: {0}
    NoTeamIdReceivedFromGrafanaOnTeamCreation(String),

    /// Permission module error
    CanNotUpdatePermissionsOnNonExistingFolder,

    /// Can not parse the datetime string to epoch time millis: {0}
    CanNotParseDateTimeToEpochTimeMillis(#[from] chrono::ParseError),
}
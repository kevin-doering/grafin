#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum FiGrafanaError {
    /// A network error occurred: {0}
    Request(#[from] reqwest::Error),

    /// An I/O error occurred: {0}
    Io(#[from] std::io::Error),

    /// Team module error: {0}
    DidNotReceiveTeamIdOnCreation(String),

    /// Permission module error
    CanNotUpdatePermissionsOnNonExistingFolder,
}
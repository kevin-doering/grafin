use clap::Args;

/// The available options for folder permissions requests
#[derive(Debug, Args, Clone)]
pub struct FolderPermissionOptions {
    /// Use it to set (override) permissions on the folder with this uid
    #[arg(short, long)]
    pub folder_uid: Option<String>,
    /// Use it to set (override) the permissions on the folder for only this team
    #[arg(short, long)]
    pub team_id: Option<u32>,
    /// Use it to set (override) the permissions level on the folder for the team
    /// Viewer = 1, Editor = 2, Admin = 4
    #[arg(short, long)]
    pub permission: Option<u8>,
}
use clap::Args;

/// The available options for a folder request to grafana
#[derive(Debug, Args, Clone)]
pub struct FolderOptions {
    /// Use it to get a team by its uid
    #[arg(short, long)]
    pub uid: Option<String>,
    /// Use it to add a folder with this title
    #[arg(short, long)]
    pub title: Option<String>,
    /// Use it to limit number of all teams you want to get
    #[arg(short, long)]
    pub limit: Option<u8>,
    /// Use it to get all team from a specified page
    #[arg(short, long)]
    pub page: Option<u8>,
}

impl FolderOptions {
    pub fn from_title(title: Option<String>) -> Self {
        Self {
            title,
            uid: None,
            limit: None,
            page: None,
        }
    }
}
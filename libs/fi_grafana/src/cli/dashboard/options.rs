use clap::Args;

/// The available option for a dashboard request to grafana
#[derive(Debug, Args, Clone)]
pub struct DashboardOptions {
    /// Use it to ref a dashboard by its instance id
    /// #[arg(short, long)]
    pub id: Option<u32>,
    ///
    #[arg(short, long)]
    pub uid: Option<String>,
    ///
    #[arg(short, long)]
    pub name: String,
    ///
    #[arg(short, long)]
    pub tags: Vec<String>,
    ///
    #[arg(short, long)]
    pub zone: String,
    ///
    #[arg(short, long)]
    pub schema_version: u16,
    ///
    #[arg(short, long)]
    pub refresh_seconds: u8,
    ///
    #[arg(short, long)]
    pub folder_uid: Option<String>,
    ///
    #[arg(short, long)]
    pub message: String,
    ///
    #[arg(short, long)]
    pub overwrite: bool,
}
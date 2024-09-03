use clap::Args;

/// The available option to post dashboards to grafana
#[derive(Debug, Args, Clone)]
pub struct DashboardOptions {
    /// Use it to ref a dashboard by its instance id
    #[arg(short, long)]
    pub id: Option<u32>,
    /// Use it to ref a dashboard by its unique identifier
    #[arg(short, long)]
    pub uid: Option<String>,
    /// Use it to set the name for the dashboard itself
    #[arg(short, long)]
    pub name: Option<String>,
    /// Add search tags to the dashboard by specifying the new state
    #[arg(short, long)]
    pub tags: Vec<String>,
    /// Specify the timezone of the dashboard [default_timezone: 'browser']
    #[arg(short, long, default_value_t = String::from("browser"))]
    pub zone: String,
    /// Use it so set the schema version for the dashboard data
    #[arg(short, long)]
    pub schema_version: Option<u16>,
    /// Declare the refresh interval in seconds
    #[arg(short, long)]
    pub refresh_seconds: Option<u8>,
    /// Specify the folder in which the dashboards belong
    #[arg(short, long)]
    pub folder_uid: Option<String>,
    /// Set a message for the latest changes been made
    #[arg(short, long)]
    pub message: Option<String>,
    /// Enable overwriting of existing dashboards with the same name or uid
    #[arg(short, long)]
    pub overwrite: bool,
    /// Use it do create a new folder for the new dashboard
    #[arg(short, long)]
    pub create_folder_name: Option<String>,
}

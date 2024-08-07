use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct PermissionOptions {
    #[arg(short, long)]
    pub folder_uid: Option<String>,
    #[arg(short, long)]
    pub team_id: Option<u32>,
    #[arg(short, long)]
    pub permission: Option<u8>,
}
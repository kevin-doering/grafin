use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct FolderOptions {
    #[arg(short, long)]
    pub uid: Option<String>,
    #[arg(short, long)]
    pub title: Option<String>,
    #[arg(short, long)]
    pub limit: Option<u8>,
    #[arg(short, long)]
    pub page: Option<u8>,
}
use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct TeamOptions {
    #[arg(short, long)]
    pub id: Option<u32>,
    #[arg(short, long)]
    pub name: Option<String>,
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub org_id: Option<u32>,
    #[arg(short, long)]
    pub folder_title: Option<String>,
    #[arg(short, long)]
    pub zero_members: Option<bool>,
    #[arg(short, long)]
    pub directory: bool,
    #[arg(short, long)]
    pub query: Option<String>,
}
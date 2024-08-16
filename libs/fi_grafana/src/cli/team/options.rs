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
    #[arg(short, long, default_value_t = false)]
    pub zero_members: bool,
    #[arg(short, long, default_value_t = false)]
    pub directory: bool,
    #[arg(short, long)]
    pub query: Option<String>,
    #[arg(short, long, default_value_t = false)]
    pub yes: bool,
}
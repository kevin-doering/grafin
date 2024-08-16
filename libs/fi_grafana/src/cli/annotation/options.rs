use clap::Args;

/// The available options for team requests
#[derive(Debug, Args, Clone)]
pub struct AnnotationOptions {
    /// Use it to get or delete a team by its id
    #[arg(short, long)]
    pub id: Option<u32>,
    /// Use it to add a team with this name
    #[arg(short, long)]
    pub name: Option<String>,
    /// Use it to add a team with this email
    #[arg(short, long)]
    pub email: Option<String>,
    /// Use it to add a team to an existing organization with this id
    #[arg(short, long)]
    pub org_id: Option<u32>,
    /// Use it to specify a folder title when the --directory flag is present (otherwise the team name is used)
    #[arg(short, long)]
    pub folder_title: Option<String>,
    /// Use it to delete all teams with zero members (confirmation required for each team with zero members)
    #[arg(short, long, default_value_t = false)]
    pub zero_members: bool,
    /// Use it to also add a folder for the team during creation
    #[arg(short, long, default_value_t = false)]
    pub directory: bool,
    /// Use it to get teams that match the given query name
    #[arg(short, long)]
    pub query: Option<String>,
    /// Use it to confirm the deletion of zero member teams upfront
    #[arg(short, long, default_value_t = false)]
    pub yes: bool,
}
use serde::Deserialize;

use crate::api::grafana::GrafanaClient;
use crate::cli::team::options::TeamOptions;
use crate::error::GrafanaCliError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchTeamsResponse {
    pub total_count: u32,
    pub teams: Vec<GetTeamResponse>,
    pub page: u32,
    pub per_page: u32,
}

impl SearchTeamsResponse {
    pub fn get_zero_member_teams(&self) -> Vec<GetTeamResponse> {
        self.teams.iter().filter(|team| team.member_count == 0).cloned().collect()
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetTeamResponse {
    pub id: u32,
    pub org_id: u32,
    pub name: String,
    pub email: String,
    pub avatar_url: String,
    pub member_count: u32,
}

pub async fn handle_get_team(grafana_client: &GrafanaClient, opt: &TeamOptions) {
    if let Some(team_id) = opt.id {
        match get_team_by_id(grafana_client, team_id).await {
            Ok(response) => {
                println!("Team:");
                println!("id: {} | name: {} | org_id: {} | email: {}", response.id, response.name, response.org_id, response.email);
                println!("avatar_url: {} | members: {}", response.avatar_url, response.member_count);
                return;
            }
            Err(error) => {
                eprintln!("{}", error);
                return;
            }
        }
    }
    match get_teams(grafana_client, opt.query.clone()).await {
        Ok(response) => {
            println!("Teams ({}):", response.total_count);
            for team in response.teams {
                println!("id: {} | name: {} | org_id: {} | members: {}", team.id, team.name, team.org_id, team.member_count);
            }
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

pub async fn get_team_by_id(grafana_client: &GrafanaClient, team_id: u32) -> Result<GetTeamResponse, GrafanaCliError> {
    match grafana_client.get(&format!("teams/{}", team_id)).await {
        Ok(response) => {
            Ok(response.json::<GetTeamResponse>().await?)
        }
        Err(error) => {
            Err(GrafanaCliError::Request(error))
        }
    }
}

pub async fn get_teams(grafana_client: &GrafanaClient, query: Option<String>) -> Result<SearchTeamsResponse, reqwest::Error> {
    if let Some(name) = query {
        match grafana_client.query("teams/search", &[("query", name)]).await {
            Ok(response) => Ok(response.json::<SearchTeamsResponse>().await?),
            Err(error) => Err(error)
        }
    } else {
        match grafana_client.get("teams/search").await {
            Ok(response) => Ok(response.json::<SearchTeamsResponse>().await?),
            Err(error) => Err(error)
        }
    }
}

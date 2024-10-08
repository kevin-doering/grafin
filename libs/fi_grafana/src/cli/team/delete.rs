use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::shell::input::{user_input, UserInput};
use crate::cli::team::get::get_teams;
use crate::cli::team::options::TeamOptions;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTeamRequest {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTeamResponse {
    message: String,
}

pub async fn handle_del_team(grafana_client: &GrafanaClient, opt: &TeamOptions) {
    if let Some(team_id) = opt.id {
        match del_team_by_id(grafana_client, team_id).await {
            Ok(response) => {
                println!("{}", response.message);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
    if opt.zero_members {
        match get_teams(grafana_client, None).await {
            Ok(response) => {
                println!("{} teams found in scope", response.total_count);
                for team_with_zero_members in response.get_zero_member_teams() {
                    let confirmation = if opt.yes {
                        Some("y".to_string())
                    } else {
                        match user_input(&format!("Delete team [{}] with [{}] members? (y/n) ", team_with_zero_members.name, team_with_zero_members.member_count)) {
                            Ok(input) => {
                                match input {
                                    UserInput::Number(_) => None,
                                    UserInput::Text(s) => Some(s)
                                }
                            }
                            Err(_) => None
                        }
                    };
                    if let Some(input) = confirmation {
                        if input.eq("y") {
                            match del_team_by_id(grafana_client, team_with_zero_members.id).await {
                                Ok(response) => {
                                    println!("{} [id: {}, name: {}]", response.message, team_with_zero_members.id, team_with_zero_members.name);
                                }
                                Err(error) => {
                                    eprintln!("{}", error);
                                }
                            }
                        } else {
                            println!("No delete confirmation. Skipping request..");
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}

pub async fn del_team_by_id(grafana_client: &GrafanaClient, team_id: u32) -> Result<DeleteTeamResponse, reqwest::Error> {
    match grafana_client.del(&format!("teams/{}", team_id)).await {
        Ok(response) => Ok(response.json::<DeleteTeamResponse>().await?),
        Err(error) => Err(error)
    }
}

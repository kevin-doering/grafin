use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::shell::input::input_dialog;
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

pub async fn handle_del_team(client: &GrafanaClient, opt: &TeamOptions) {
    if let Some(team_id) = opt.id {
        match del_team_by_id(&client, team_id).await {
            Ok(response) => {
                println!("{}", response.message);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
    if let Some(member_zeroes) = opt.zero_members {
        if member_zeroes {
            match get_teams(&client, None).await {
                Ok(response) => {
                    println!("{} teams found in scope", response.total_count);
                    for team_with_zero_members in response.get_zero_member_teams() {
                        let confirmation = input_dialog(&format!("Delete team [{}] with [{}] members? (y/n)", team_with_zero_members.name, team_with_zero_members.member_count));
                        if let Some(input) = confirmation {
                            if input.eq("y") {
                                match del_team_by_id(&client, team_with_zero_members.id).await {
                                    Ok(response) => {
                                        println!("{}", response.message);
                                    }
                                    Err(error) => {
                                        eprintln!("{}", error);
                                    }
                                }
                            } else {
                                println!("No confirmation. Skipping..");
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
}

pub async fn del_team_by_id(client: &GrafanaClient, team_id: u32) -> Result<DeleteTeamResponse, reqwest::Error> {
    match client.del(&format!("teams/{}", team_id)).await {
        Ok(response) => Ok(response.json::<DeleteTeamResponse>().await?),
        Err(error) => Err(error)
    }
}

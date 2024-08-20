use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::folder::add::add_folder;
use crate::cli::folder::permission::set::{FolderPermissionItem, set_folder_permissions};
use crate::cli::shell::input::prompt_option;
use crate::cli::team::options::TeamOptions;
use crate::error::GrafanaCliError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTeamRequest {
    pub name: String,
    pub email: Option<String>,
    pub org_id: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTeamResponse {
    pub message: String,
    pub team_id: Option<u32>,
}

pub async fn handle_add_team(grafana_client: &GrafanaClient, opt: &TeamOptions) {
    let team_name = prompt_option("Enter a team name: ", &opt.name);
    let (admin_team_id, viewer_team_id) = if let Some(name) = &team_name {
        let admin_team_id = match add_team(grafana_client, name.clone(), opt).await {
            Ok(response) => {
                response.team_id
            }
            Err(error) => {
                eprintln!("{}", error);
                None
            }
        };
        let viewer_team_id = match add_team(grafana_client, format!("{}-{}", name, "Viewer"), opt).await {
            Ok(response) => {
                response.team_id
            }
            Err(error) => {
                eprintln!("{}", error);
                None
            }
        };
        (admin_team_id, viewer_team_id)
    } else {
        (None, None)
    };
    let folder_uid = if let Some(_) = admin_team_id {
        if let Some(folder_title) = &opt.folder_title {
            // todo: reduce duplication
            match add_folder(grafana_client, folder_title.clone()).await {
                Ok(response) => {
                    Some(response.uid)
                }
                Err(error) => {
                    eprintln!("{}", error);
                    None
                }
            }
        } else {
            if opt.directory {
                if let Some(team_name) = &team_name {
                    match add_folder(grafana_client, team_name.clone()).await {
                        Ok(response) => {
                            Some(response.uid)
                        }
                        Err(error) => {
                            eprintln!("{}", error);
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
    } else {
        None
    };
    if let Some(folder_uid) = folder_uid {
        let mut items = FolderPermissionItem::default_permissions_items();
        if let Some(admin) = admin_team_id {
            items.push(FolderPermissionItem::team(admin, 4));
        }
        if let Some(viewer) = viewer_team_id {
            items.push(FolderPermissionItem::team(viewer, 1));
        }
        match set_folder_permissions(grafana_client, Some(folder_uid), items).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

async fn add_team(grafana_client: &GrafanaClient, name: String, opt: &TeamOptions) -> Result<AddTeamResponse, GrafanaCliError> {
    let request = AddTeamRequest {
        name: name.clone(),
        email: opt.email.clone(),
        org_id: opt.org_id,
    };
    match post_add_team(grafana_client, &request).await {
        Ok(response) => {
            if let Some(team_id) = response.team_id {
                println!("{} [id: {}, name: {}]", response.message, team_id, name);
                Ok(response)
            } else {
                println!("No team created! Reason: {}", response.message);
                Err(GrafanaCliError::NoTeamIdReceivedFromGrafanaOnTeamCreation("No team id received from grafana on team creation!".to_string()))
            }
        }
        Err(error) => {
            eprintln!("No team created! error: {}", error);
            Err(GrafanaCliError::Request(error))
        }
    }
}

async fn post_add_team(grafana_client: &GrafanaClient, request: &AddTeamRequest) -> Result<AddTeamResponse, reqwest::Error> {
    match grafana_client
        .post("teams", request).await {
        Ok(response) => Ok(response.json::<AddTeamResponse>().await?),
        Err(error) => Err(error)
    }
}
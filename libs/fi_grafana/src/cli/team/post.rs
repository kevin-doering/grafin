use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::folder::permission::post::{FolderPermissionItem, post_permission};
use crate::cli::folder::post::post_folder;
use crate::cli::shell::input::input_dialog;
use crate::cli::team::options::TeamOptions;
use crate::error::FiGrafanaError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamRequest {
    pub name: String,
    pub email: Option<String>,
    pub org_id: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamResponse {
    pub message: String,
    pub team_id: Option<u32>,
}

pub async fn handle_post_team(client: &GrafanaClient, opt: &TeamOptions) {
    let team_name = prompt_option("Enter a team name: ", opt.name.clone());
    let (admin_team_id, viewer_team_id) = if let Some(name) = &team_name {
        let admin_team_id = match post_team(client, name.clone(), opt).await {
            Ok(response) => {
                response.team_id
            }
            Err(error) => {
                eprintln!("{}", error);
                None
            }
        };
        let viewer_team_id = match post_team(client, format!("{}-{}", name, "Viewer"), opt).await {
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
            match post_folder(client, folder_title.clone()).await {
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
                    match post_folder(client, team_name.clone()).await {
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
        match post_permission(client, Some(folder_uid), items).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

pub fn prompt_option(prompt: &str, opt: Option<String>) -> Option<String> {
    if opt.is_none() {
        input_dialog(prompt)
    } else {
        opt.clone()
    }
}

async fn post_team(client: &GrafanaClient, name: String, opt: &TeamOptions) -> Result<CreateTeamResponse, FiGrafanaError> {
    let request = CreateTeamRequest {
        name: name.clone(),
        email: opt.email.clone(),
        org_id: opt.org_id,
    };
    match create_team(client, &request).await {
        Ok(response) => {
            if let Some(team_id) = response.team_id {
                println!("{} [id: {}, name: {}]", response.message, team_id, name);
                Ok(response)
            } else {
                println!("No team created! Reason: {}", response.message);
                Err(FiGrafanaError::DidNotReceiveTeamIdOnCreation("No team id received from the server!".to_string()))
            }
        }
        Err(error) => {
            eprintln!("No team created! error: {}", error);
            Err(FiGrafanaError::Request(error))
        }
    }
}

async fn create_team(client: &GrafanaClient, request: &CreateTeamRequest) -> Result<CreateTeamResponse, reqwest::Error> {
    match client
        .post("teams", request).await {
        Ok(response) => Ok(response.json::<CreateTeamResponse>().await?),
        Err(error) => Err(error)
    }
}
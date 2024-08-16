use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::folder::permission::options::FolderPermissionOptions;
use crate::cli::team::post::prompt_option;
use crate::error::FiGrafanaError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderPermissionRequest {
    pub items: Vec<FolderPermissionItem>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FolderPermissionItem {
    pub role: Option<String>,
    pub team_id: Option<u32>,
    pub user_id: Option<u32>,
    pub permission: u8,
}

impl FolderPermissionItem {
    pub fn default_permissions_items() -> Vec<FolderPermissionItem> {
        let viewer = FolderPermissionItem::role("Viewer", 1);
        let editor = FolderPermissionItem::role("Editor", 2);
        let admin = FolderPermissionItem::role("Admin", 4);
        vec![viewer, editor, admin]
    }

    pub fn role(role: &str, permission: u8) -> Self {
        Self {
            role: Some(role.to_string()),
            team_id: None,
            user_id: None,
            permission,
        }
    }

    pub fn team(team_id: u32, permission: u8) -> Self {
        Self {
            role: None,
            team_id: Some(team_id),
            user_id: None,
            permission,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderPermissionResponse {
    pub message: String,
}

pub async fn handle_post_permission(client: &GrafanaClient, opt: &FolderPermissionOptions) {
    let mut items = FolderPermissionItem::default_permissions_items();
    if let Some(team_id) = opt.team_id {
        if let Some(permission) = opt.permission {
            items.push(FolderPermissionItem::team(team_id, permission));
        }
    }
    match post_permission(client, opt.folder_uid.clone(), items).await {
        Ok(_) => {}
        Err(_) => {}
    }
}

pub async fn post_permission(client: &GrafanaClient, folder_uid: Option<String>, items: Vec<FolderPermissionItem>) -> Result<UpdateFolderPermissionResponse, FiGrafanaError> {
    let update = UpdateFolderPermissionRequest { items: items.clone() };
    let folder_uid = prompt_option("Enter the folder_uid: ", folder_uid);
    if let Some(folder_uid) = folder_uid {
        match update_folder_permission(client, &update, folder_uid.clone()).await {
            Ok(response) => {
                println!("{} [uid: {}]", response.message, folder_uid);
                for item in items {
                    if let Some(role) = item.role {
                        print!("role: {} ", role);
                    }
                    if let Some(team_id) = item.team_id {
                        print!("team_id: {} ", team_id);
                    }
                    if let Some(user_id) = item.user_id {
                        print!("user_id: {} ", user_id);
                    }
                    println!("permission: {}", item.permission);
                }
                Ok(response)
            }
            Err(error) => {
                eprintln!("{}", error);
                Err(FiGrafanaError::Request(error))
            }
        }
    } else {
        Err(FiGrafanaError::CanNotUpdatePermissionsOnNonExistingFolder)
    }
}

async fn update_folder_permission(client: &GrafanaClient, request: &UpdateFolderPermissionRequest, folder_uid: String) -> Result<UpdateFolderPermissionResponse, reqwest::Error> {
    // println!("{:#?}", request);
    match client.post(&format!("folders/{}/permissions", folder_uid), request).await {
        Ok(response) => Ok(response.json::<UpdateFolderPermissionResponse>().await?),
        Err(error) => Err(error)
    }
}
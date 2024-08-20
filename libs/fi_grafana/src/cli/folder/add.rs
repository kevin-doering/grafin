use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::folder::options::FolderOptions;
use crate::cli::shell::input::prompt_option;
use crate::error::GrafanaCliError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFolderRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFolderResponse {
    pub id: u32,
    pub uid: String,
    pub title: String,
    pub url: String,
    pub has_acl: bool,
    pub can_save: bool,
    pub can_edit: bool,
    pub can_admin: bool,
    pub created_by: String,
    pub created: String,
    pub updated_by: String,
    pub updated: String,
    pub version: u32,
}

pub async fn handle_add_folder(grafana_client: &GrafanaClient, opt: &FolderOptions) {
    let title = prompt_option("Enter the folder title: ", &opt.title);
    if let Some(title) = title {
        match add_folder(grafana_client, title).await {
            Ok(response) => {
                println!("folder url: {}", response.url);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}

pub async fn add_folder(grafana_client: &GrafanaClient, title: String) -> Result<AddFolderResponse, GrafanaCliError> {
    let request = AddFolderRequest { title: title.clone() };
    match post_add_folder(grafana_client, &request).await {
        Ok(response) => {
            println!("Folder created [uid: {}, title: {}]", response.uid, title);
            Ok(response)
        }
        Err(error) => {
            eprintln!("{}", error);
            Err(GrafanaCliError::Request(error))
        }
    }
}

async fn post_add_folder(grafana_client: &GrafanaClient, request: &AddFolderRequest) -> Result<AddFolderResponse, reqwest::Error> {
    match grafana_client.post("folders", request).await {
        Ok(response) => Ok(response.json::<AddFolderResponse>().await?),
        Err(error) => Err(error)
    }
}
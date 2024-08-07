use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::folder::options::FolderOptions;
use crate::cli::team::post::prompt_option;
use crate::error::FiGrafanaError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderResponse {
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

pub async fn handle_post_folder(client: &GrafanaClient, opt: &FolderOptions) {
    let title = prompt_option("Enter the folder title: ", opt.title.clone());
    if let Some(title) = title {
        match post_folder(client, title).await {
            Ok(response) => {
                println!("folder url: {}", response.url);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}

pub async fn post_folder(client: &GrafanaClient, title: String) -> Result<CreateFolderResponse, FiGrafanaError> {
    let request = CreateFolderRequest { title: title.clone() };
    match create_folder(client, &request).await {
        Ok(response) => {
            println!("Folder created [uid: {}, title: {}]", response.uid, title);
            Ok(response)
        }
        Err(error) => {
            eprintln!("{}", error);
            Err(FiGrafanaError::Request(error))
        }
    }
}

async fn create_folder(client: &GrafanaClient, request: &CreateFolderRequest) -> Result<CreateFolderResponse, reqwest::Error> {
    match client.post("folders", request).await {
        Ok(response) => Ok(response.json::<CreateFolderResponse>().await?),
        Err(error) => Err(error)
    }
}
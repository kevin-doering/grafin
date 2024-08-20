use serde::Deserialize;

use crate::api::grafana::GrafanaClient;
use crate::cli::folder::options::FolderOptions;
use crate::error::GrafanaCliError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryFolderResponse {
    id: u32,
    uid: String,
    title: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetFolderResponse {
    id: u32,
    uid: String,
    title: String,
    url: String,
    has_acl: bool,
    can_save: bool,
    can_edit: bool,
    can_admin: bool,
    created_by: String,
    created: String,
    updated_by: String,
    updated: String,
    version: u8,
    // if nested folders are enabled and the folder is nested
    parent_uid: Option<String>,
    parents: Option<Vec<String>>,
}

pub async fn handle_get_folder(grafana_client: &GrafanaClient, opt: &FolderOptions) {
    if let Some(uid) = &opt.uid {
        match get_folder_by_uid(grafana_client, uid.clone()).await {
            Ok(response) => {
                println!("Folder:");
                println!("id: {} | uid: {} | title: {}", response.id, response.uid, response.title);
                println!("created: {} | updated: {}", response.created, response.updated);
                println!("url: {}", response.url);
                println!("version: {} | parent: {:?}", response.version, response.parent_uid);
                return;
            }
            Err(error) => {
                eprintln!("{}", error);
                return;
            }
        }
    }
    let limit = opt.limit.unwrap_or(0);
    let page = opt.page.unwrap_or(0);
    match query_folders(grafana_client, limit, page).await {
        Ok(response) => {
            println!("Folders ({}):", response.len());
            for folder in response {
                println!("id: {} | uid: {} | title: {}", folder.id, folder.uid, folder.title);
            }
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

async fn query_folders(grafana_client: &GrafanaClient, limit: u8, page: u8) -> Result<Vec<QueryFolderResponse>, GrafanaCliError> {
    match grafana_client.query("folders", &[("limit", limit), ("page", page)]).await {
        Ok(response) => {
            Ok(response.json::<Vec<QueryFolderResponse>>().await?)
        }
        Err(error) => {
            Err(GrafanaCliError::Request(error))
        }
    }
}

async fn get_folder_by_uid(grafana_client: &GrafanaClient, uid: String) -> Result<GetFolderResponse, GrafanaCliError> {
    match grafana_client.get(&format!("folders/{}", uid)).await {
        Ok(response) => {
            Ok(response.json::<GetFolderResponse>().await?)
        }
        Err(error) => {
            Err(GrafanaCliError::Request(error))
        }
    }
}
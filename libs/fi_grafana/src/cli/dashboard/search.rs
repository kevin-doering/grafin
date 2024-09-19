use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::error::GrafanaCliError;

pub const DASH_DB_TYPE: &'static str = "dash-db";
pub const DASH_FOLDER_TYPE: &'static str = "dash-folder";

pub const TIME_SERIES_PANEL_TYPE: &'static str = "timeseries";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchDashTypeRequest {
    pub r#type: Option<String>,
    pub query: Option<String>,
    #[serde(rename = "folderUIDs")]
    pub folder_uids: Option<Vec<String>>,
    #[serde(rename = "dashboardUIDs")]
    pub dashboard_uids: Option<Vec<String>>,
}

impl SearchDashTypeRequest {
    pub fn type_query(r#type: String, query: String) -> Self {
        Self {
            r#type: Some(r#type),
            query: Some(query.clone()),
            folder_uids: None,
            dashboard_uids: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashTypeResponse {
    pub id: u32,
    pub uid: String,
    pub title: String,
    pub uri: String,
    pub url: String,
    pub slug: String,
    pub r#type: String,
    pub tags: Vec<String>,
    pub is_starred: bool,
    pub sort_meta: u16,
    /// This field is only present when the dash-folder or dash-db type is nested within another folder
    pub folder_id: Option<u32>,
    /// This field is only present when the dash-folder or dash-db type is nested within another folder
    pub folder_uid: Option<String>,
    /// This field is only present when the dash-folder or dash-db type is nested within another folder
    pub folder_title: Option<String>,
    /// This field is only present when the dash-folder or dash-db type is nested within another folder
    pub folder_url: Option<String>,
}

pub async fn search_for_dash_types(grafana_client: &GrafanaClient, request: SearchDashTypeRequest) -> Result<Vec<DashTypeResponse>, GrafanaCliError> {
    let resource = serde_url_params::to_string(&request)?;
    match grafana_client.get(&format!("search?{resource}")).await {
        Ok(response) => Ok(response.json::<Vec<DashTypeResponse>>().await?),
        Err(error) => Err(GrafanaCliError::Request(error)),
    }
}

pub async fn get_dash_type_uids(grafana_client: &GrafanaClient, request: SearchDashTypeRequest) -> Result<Vec<String>, GrafanaCliError> {
    Ok(search_for_dash_types(grafana_client, request).await?.iter().map(|dash_type| { dash_type.uid.clone() }).collect())
}
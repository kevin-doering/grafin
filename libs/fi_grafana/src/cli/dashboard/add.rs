use serde::{Deserialize, Serialize};

use crate::api::grafana::GrafanaClient;
use crate::cli::dashboard::options::DashboardOptions;
use crate::error::GrafanaCliError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDashboardRequest {
    /// The dashboard schema to create
    pub dashboard: PostDashboard,
    /// The folder where the dashboard lives
    pub folder_uid: Option<String>,
    /// The reasoning behind the change
    pub message: String,
    /// If overwriting existing dashboards with same title or uid is the wish
    pub overwrite: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDashboard {
    /// The incremental id of the grafana instance
    pub id: Option<u32>,
    /// The unique identifier across instances
    pub uid: Option<String>,
    /// The unique dashboard name within a folder
    pub title: String,
    /// The associated search tags
    pub tags: Vec<String>,
    /// The desired dashboard timezone
    pub timezone: String,
    /// The defined schema version
    pub schema_version: u16,
    /// The refresh rate in default seconds
    pub refresh: String,
}

impl AddDashboardRequest {
    pub fn new_by_option(opt: &DashboardOptions) -> Self {
        Self {
            dashboard: PostDashboard {
                id: opt.id,
                uid: opt.uid.clone(),
                title: opt.name.clone(),
                tags: opt.tags.clone(),
                timezone: opt.zone.clone(),
                schema_version: opt.schema_version,
                refresh: format!("{}s", opt.refresh_seconds),
            },
            folder_uid: opt.folder_uid.clone(),
            message: opt.message.clone(),
            overwrite: opt.overwrite,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDashboardSuccessResponse {
    pub id: u32,
    pub uid: String,
    pub url: String,
    pub status: String,
    pub version: u16,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDashboardErrorResponse {
    pub message: String,
    pub status: String,
}

pub async fn handle_add_dashboard(grafana_client: &GrafanaClient, opt: &DashboardOptions) {
    add_dashboard(grafana_client, opt).await;
}

async fn add_dashboard(grafana_client: &GrafanaClient, opt: &DashboardOptions) {
    let request = AddDashboardRequest::new_by_option(opt);
    println!("adding dashboard with name: {} into unique folder: '{:#?}'", request.dashboard.title, request.folder_uid);
    match post_add_dashboard(grafana_client, &request).await {
        Ok((success, error)) => {
            if let Some(response) = success {
                println!("status: {} [id: {}, uid: {}, version: {}]", response.status, response.id, response.uid, response.version);
                println!("url: {} | slug: {}", response.url, response.slug);
            }
            if let Some(response) = error {
                eprintln!("status: {} message: {}", response.status, response.message);
            }
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}

pub type AddDashboardResponse = (Option<AddDashboardSuccessResponse>, Option<AddDashboardErrorResponse>);

async fn post_add_dashboard(
    grafana_client: &GrafanaClient,
    request: &AddDashboardRequest,
) -> Result<AddDashboardResponse, GrafanaCliError> {
    match grafana_client.post("dashboards/db", request).await {
        Ok(response) => {
            let body = response.text().await.map_err(GrafanaCliError::Request)?;
            if let Ok(success) = serde_json::from_str::<AddDashboardSuccessResponse>(&body) {
                Ok((Some(success), None))
            } else if let Ok(error) = serde_json::from_str::<AddDashboardErrorResponse>(&body) {
                Ok((None, Some(error)))
            } else {
                Err(GrafanaCliError::InvalidResponseFormat(body))
            }
        }
        Err(error) => Err(GrafanaCliError::Request(error)),
    }
}


use serde::Deserialize;

use crate::api::grafana::GrafanaClient;
use crate::cli::dashboard::options::DashboardOptions;
use crate::error::GrafanaCliError;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDashboardResponse {
    pub dashboard: GetDashboard,
    pub meta: GetDashboardMeta,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDashboard {
    /// The incremental id of the grafana instance
    pub id: u32,
    /// The unique identifier across instances
    pub uid: String,
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDashboardMeta {
    pub is_starred: bool,
    pub url: String,
    pub folder_id: u32,
    pub folder_uid: String,
}

pub async fn handle_get_dashboard(grafana_client: &GrafanaClient, opt: &DashboardOptions) {
    if let Some(uid) = &opt.uid {
        match get_dashboard_by_uid(grafana_client, uid.clone()).await {
            Ok(response) => {
                println!("Dashboard:");
                println!("id: {} | uid: {} | name: {}", response.dashboard.id, response.dashboard.uid, response.dashboard.title);
                println!("timezone: {} | refresh: {} | schema_version: {}", response.dashboard.timezone, response.dashboard.refresh, response.dashboard.schema_version);
                println!("tags: [{}]", response.dashboard.tags.join(", "));
                println!("Folder:");
                println!("id: {} | uid: {} | is_starred: {}", response.meta.folder_id, response.meta.folder_uid, response.meta.is_starred);
                println!("url: {}", response.meta.url);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}

async fn get_dashboard_by_uid(grafana_client: &GrafanaClient, uid: String) -> Result<GetDashboardResponse, GrafanaCliError> {
    match grafana_client.get(&format!("dashboards/uid/{}", uid)).await {
        Ok(response) => Ok(response.json::<GetDashboardResponse>().await?),
        Err(error) => Err(GrafanaCliError::Request(error))
    }
}
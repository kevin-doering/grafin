use serde::Deserialize;

use crate::api::grafana::GrafanaClient;
use crate::cli::dashboard::options::DashboardOptions;
use crate::error::GrafanaCliError;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDashboardResponse {
    pub dashboard: GetDashboard,
    pub meta: GetDashboardMeta,
}

/// Some conditional fields of the resource are omitted (there are more to work with)
#[derive(Debug, Deserialize, Clone)]
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
    /// The dashboard version
    pub version: u8,
    /// The available panels of the dashboard
    pub panels: Option<Vec<GetPanel>>,
}

/// Some conditional fields of the resource are omitted (there are more to work with)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetPanel {
    pub id: u32,
    pub title: String,
    pub r#type: String,
    pub datasource: GetPanelDataSource,
    pub grid_pos: GetPanelGridPosition,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetPanelDataSource {
    pub r#type: String,
    pub uid: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetPanelGridPosition {
    pub h: u16,
    pub w: u16,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDashboardMeta {
    pub r#type: String,
    pub can_save: bool,
    pub can_edit: bool,
    pub can_admin: bool,
    pub can_star: bool,
    pub can_delete: bool,
    pub slug: String,
    pub url: String,
    pub expires: String,
    pub created: String,
    pub updated: String,
    pub updated_by: String,
    pub created_by: String,
    pub version: u16,
    pub has_acl: bool,
    pub is_folder: bool,
    pub folder_id: u32,
    pub folder_uid: String,
    pub folder_title: String,
    pub folder_url: String,
    pub provisioned: bool,
    pub provisioned_external_id: String,
    pub annotations_permissions: AnnotationsPermissions,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationsPermissions {
    pub dashboard: DashboardAnnotationPermissions,
    pub organization: OrganizationAnnotationPermissions,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DashboardAnnotationPermissions {
    pub can_add: bool,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAnnotationPermissions {
    pub can_add: bool,
    pub can_edit: bool,
    pub can_delete: bool,
}

pub async fn handle_get_dashboard(grafana_client: &GrafanaClient, opt: &DashboardOptions) {
    if let Some(uid) = &opt.uid {
        match get_dashboard_by_uid(grafana_client, uid).await {
            Ok(response) => {
                println!("Dashboard:");
                println!("id: {} | uid: {} | name: {}", response.dashboard.id, response.dashboard.uid, response.dashboard.title);
                println!("timezone: {} | version: {} | schema_version: {}", response.dashboard.timezone, response.dashboard.version, response.dashboard.schema_version);
                println!("tags: [{}]", response.dashboard.tags.join(", "));
                println!("Folder:");
                println!("id: {} | uid: {} | title: {}", response.meta.folder_id, response.meta.folder_uid, response.meta.folder_title);
                println!("url: {}", response.meta.url);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}

pub async fn get_dashboard_by_uid(grafana_client: &GrafanaClient, uid: &String) -> Result<GetDashboardResponse, GrafanaCliError> {
    match grafana_client.get(&format!("dashboards/uid/{}", uid)).await {
        Ok(response) => Ok(response.json::<GetDashboardResponse>().await?),
        Err(error) => Err(GrafanaCliError::Request(error))
    }
}
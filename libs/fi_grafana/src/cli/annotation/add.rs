use serde::Deserialize;
use serde_with::serde_derive::Serialize;

use crate::api::grafana::GrafanaClient;
use crate::cli::annotation::options::AnnotationOptions;
use crate::cli::shell::date::{DATETIME_FORMAT, parse_datetime_to_epoch_time_millis};
use crate::cli::shell::input::prompt_option;
use crate::error::GrafanaCliError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAnnotationRequest {
    /// When dashboard_uid and panel_id are not set, it will be an organizational annotation
    pub dashboard_uid: Option<String>,
    /// When dashboard_uid and panel_id are not set, it will be an organizational annotation
    pub panel_id: Option<u32>,
    /// Epoch time in millisecond resolution
    pub time: Option<i64>,
    /// Epoch time in millisecond resolution (when setting the time_end field, it will be a regional annotation)
    pub time_end: Option<i64>,
    /// Tags associated with this annotation
    pub tags: Vec<String>,
    /// Description of the annotation
    pub text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAnnotationResponse {
    /// When the annotation was created the response contains its id
    id: Option<u32>,
    /// The status message whether the annotation was created or not
    message: String,
}

pub async fn handle_add_annotation(grafana_client: &GrafanaClient, opt: &AnnotationOptions) {
    if opt.organizational {
        add_organizational_annotation(grafana_client, opt).await;
    } else {
        match add_annotation_to_dashboard_panel(grafana_client, opt).await {
            Ok(response) => {
                if let Some(id) = response.id {
                    println!("{} [id: {}]", response.message, id);
                } else {
                    println!("{}", response.message);
                }
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}

pub async fn add_organizational_annotation(grafana_client: &GrafanaClient, opt: &AnnotationOptions) {}


pub async fn add_annotation_to_dashboard_panel(grafana_client: &GrafanaClient, opt: &AnnotationOptions) -> Result<AddAnnotationResponse, GrafanaCliError> {
    let dashboard_uid = prompt_option("Enter a dashboard_uid: ", &opt.dashboard_uid);
    let panel_id = prompt_option("Enter a panel_id: ", &opt.panel_id);
    let time = prompt_option(&format!("Enter a start_datetime [format: {}]: ", DATETIME_FORMAT), &opt.start_datetime);
    let time = parse_datetime_to_epoch_time_millis(&time);
    let time_end = parse_datetime_to_epoch_time_millis(&opt.end_datetime);
    if opt.start_datetime.is_some() && time.is_none() {
        return Err(GrafanaCliError::CanNotParseTheStartDateTimeToEpochTimeMillis);
    }
    if opt.end_datetime.is_some() && time_end.is_none() {
        return Err(GrafanaCliError::CanNotParseTheEndDateTimeToEpochTimeMillis);
    }
    if let (Some(dashboard_uid), Some(panel_id)) = (&dashboard_uid, panel_id) {
        println!("Adding an annotation to the dashboard's panel: [dashboard_uid: {}, panel_id: {}]", dashboard_uid, panel_id);
    }
    let request = AddAnnotationRequest {
        dashboard_uid,
        panel_id,
        time,
        time_end,
        tags: opt.tags.clone(),
        text: opt.comment.clone(),
    };
    post_add_annotation_to_dashboard_panel(grafana_client, &request).await
}

async fn post_add_annotation_to_dashboard_panel(grafana_client: &GrafanaClient, request: &AddAnnotationRequest) -> Result<AddAnnotationResponse, GrafanaCliError> {
    match grafana_client
        .post("annotations", request).await {
        Ok(response) => Ok(response.json::<AddAnnotationResponse>().await?),
        Err(error) => Err(GrafanaCliError::Request(error))
    }
}
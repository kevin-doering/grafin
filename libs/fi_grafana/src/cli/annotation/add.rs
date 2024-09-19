use serde::Deserialize;
use serde_with::serde_derive::Serialize;

use crate::api::grafana::GrafanaClient;
use crate::cli::annotation::options::AnnotationOptions;
use crate::cli::dashboard::get::get_dashboard_by_uid;
use crate::cli::dashboard::search::{DASH_DB_TYPE, DASH_FOLDER_TYPE, get_dash_type_uids, SearchDashTypeRequest, TIME_SERIES_PANEL_TYPE};
use crate::cli::shell::date::{DATETIME_FORMAT, from_datetime_to_epoch_time_millis, parse_datetime_to_epoch_time_millis};
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
    if opt.all_panel_where_dashboard_name_is_like.is_some() || opt.within_folders_where_folder_name_is_like.is_some() {
        match add_annotations_to_all_panel_within_the_specified_dash_type_scope(grafana_client, opt).await {
            Ok(response) => {
                for r in &response {
                    if let Some(id) = r.id {
                        println!("id: {}", id);
                        println!("message: {}", r.message);
                    }
                }
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
        return;
    }
    if opt.organizational {
        return match add_organizational_annotation(grafana_client, opt).await {
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
        };
    }
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

async fn add_annotations_to_all_panel_within_the_specified_dash_type_scope(grafana_client: &GrafanaClient, opt: &AnnotationOptions) -> Result<Vec<AddAnnotationResponse>, GrafanaCliError> {
    let time = if let Some(start) = &opt.start_datetime {
        Some(from_datetime_to_epoch_time_millis(start)?)
    } else {
        return Err(GrafanaCliError::CanNotParseTheStartDateTimeToEpochTimeMillis);
    };
    let time_end = if let Some(end) = &opt.end_datetime {
        Some(from_datetime_to_epoch_time_millis(end)?)
    } else {
        return Err(GrafanaCliError::CanNotParseTheEndDateTimeToEpochTimeMillis);
    };
    let folder_uids = if let Some(folder_name) = &opt.within_folders_where_folder_name_is_like {
        let request = SearchDashTypeRequest::type_query(DASH_FOLDER_TYPE.to_string(), folder_name.clone());
        get_dash_type_uids(grafana_client, request).await?
    } else {
        vec![]
    };
    let dashboard_uids = if let Some(dashboard_name) = &opt.all_panel_where_dashboard_name_is_like {
        let request = SearchDashTypeRequest::type_query(DASH_DB_TYPE.to_string(), dashboard_name.clone());
        get_dash_type_uids(grafana_client, request).await?
    } else {
        vec![]
    };
    let named_dashboard_uids = get_dash_type_uids(grafana_client, SearchDashTypeRequest {
        r#type: Some(DASH_DB_TYPE.to_string()),
        query: None,
        folder_uids: Some(folder_uids),
        dashboard_uids: Some(dashboard_uids),
    }).await?;
    Ok(add_annotation_to_all_panels_with_type(grafana_client, TIME_SERIES_PANEL_TYPE, named_dashboard_uids, opt, time, time_end).await?)
}

async fn add_annotation_to_all_panels_with_type(
    grafana_client: &GrafanaClient,
    panel_type: &str,
    dashboard_uids: Vec<String>,
    opt: &AnnotationOptions,
    time: Option<i64>,
    time_end: Option<i64>,
) -> Result<Vec<AddAnnotationResponse>, GrafanaCliError> {
    let mut responses = vec![];
    for dashboard_uid in &dashboard_uids {
        let response = get_dashboard_by_uid(grafana_client, dashboard_uid).await?;
        if let Some(panels) = response.dashboard.panels {
            for panel in panels {
                if panel.r#type.eq(panel_type) {
                    let request = AddAnnotationRequest {
                        dashboard_uid: Some(dashboard_uid.clone()),
                        panel_id: Some(panel.id.clone()),
                        time,
                        time_end,
                        tags: opt.tags.clone(),
                        text: opt.comment.clone(),
                    };
                    responses.push(post_add_annotation(grafana_client, &request).await?);
                }
            }
        }
    }
    Ok(responses)
}


async fn add_organizational_annotation(grafana_client: &GrafanaClient, opt: &AnnotationOptions) -> Result<AddAnnotationResponse, GrafanaCliError> {
    if opt.dashboard_uid.is_some() {
        println!("Ignoring the 'dashboard_uid' because the 'organizational' flag is present which does not require a dashboard reference");
    }
    if opt.panel_id.is_some() {
        println!("Ignoring the 'panel_id' because the 'organizational' flag is present which does not require a panel reference");
    }
    if opt.start_datetime.is_some() {
        println!("Ignoring the 'start_datetime' because the 'organizational' flag is present which does not require a specified datetime");
    }
    if opt.end_datetime.is_some() {
        println!("Ignoring the 'end_datetime' because the 'organizational' flag is present which does not require a specified datetime");
    }
    let request = AddAnnotationRequest {
        dashboard_uid: None,
        panel_id: None,
        time: None,
        time_end: None,
        tags: opt.tags.clone(),
        text: opt.comment.clone(),
    };
    post_add_annotation(grafana_client, &request).await
}


async fn add_annotation_to_dashboard_panel(grafana_client: &GrafanaClient, opt: &AnnotationOptions) -> Result<AddAnnotationResponse, GrafanaCliError> {
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
    post_add_annotation(grafana_client, &request).await
}

async fn post_add_annotation(grafana_client: &GrafanaClient, request: &AddAnnotationRequest) -> Result<AddAnnotationResponse, GrafanaCliError> {
    match grafana_client
        .post("annotations", request).await {
        Ok(response) => Ok(response.json::<AddAnnotationResponse>().await?),
        Err(error) => Err(GrafanaCliError::Request(error))
    }
}
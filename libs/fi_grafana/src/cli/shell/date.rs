use chrono::NaiveDateTime;

use crate::error::GrafanaCliError;

pub const DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M";

pub fn parse_datetime_to_epoch_time_millis(datetime: &Option<String>) -> Option<i64> {
    if let Some(datetime) = datetime {
        match from_datetime_to_epoch_time_millis(datetime) {
            Ok(epoch_time_millis) => Some(epoch_time_millis),
            Err(error) => {
                eprintln!("{}", error);
                None
            }
        }
    } else {
        None
    }
}

fn from_datetime_to_epoch_time_millis(datetime_str: &str) -> Result<i64, GrafanaCliError> {
    let datetime = NaiveDateTime::parse_from_str(datetime_str, DATETIME_FORMAT);
    match datetime {
        Ok(datetime) => Ok(datetime.and_utc().timestamp_millis()),
        Err(error) => Err(GrafanaCliError::CanNotParseDateTimeToEpochTimeMillis(error))
    }
}
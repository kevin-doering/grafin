use chrono::NaiveDateTime;

use crate::error::GrafanaCliError;

const DATETIME_FORMAT: &'static str = "%Y-%m-%d %H:%M";

pub fn from_datetime_to_epoch_time_millis(datetime_str: &str) -> Result<i64, GrafanaCliError> {
    let datetime = NaiveDateTime::parse_from_str(datetime_str, DATETIME_FORMAT);
    match datetime {
        Ok(datetime) => Ok(datetime.and_utc().timestamp_millis()),
        Err(error) => Err(GrafanaCliError::CanNotParseDateTimeToEpochTimeMillis(error))
    }
}
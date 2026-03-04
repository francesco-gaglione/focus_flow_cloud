pub fn timestamp_to_datetime(timestamp: Option<i64>) -> TaskMapperResult<Option<DateTime<Utc>>> {
    match timestamp {
        Some(ts) => {
            let datetime =
                DateTime::from_timestamp(ts, 0).ok_or(TaskMapperError::InvalidTimestamp(ts))?;
            Ok(Some(datetime))
        }
        None => Ok(None),
    }
}

/// Convert DateTime<Utc> to Unix timestamp (seconds)
pub fn datetime_to_timestamp(date: Option<DateTime<Utc>>) -> Option<i64> {
    date.map(|dt| dt.timestamp())
}

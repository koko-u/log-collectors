use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}
impl fmt::Display for DateTimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateTimeRange {
                from: Some(from),
                until: Some(until),
            } => write!(f, "{from}..{until}"),
            DateTimeRange {
                from: Some(from),
                until: None,
            } => write!(f, "{from}.."),
            DateTimeRange {
                from: None,
                until: Some(until),
            } => write!(f, "..{until}"),
            DateTimeRange {
                from: None,
                until: None,
            } => write!(f, ".."),
        }
    }
}

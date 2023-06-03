use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogResponse {
    pub user_agent: String,
    pub response_time: u32,
    pub timestamp: DateTime<Utc>,
}

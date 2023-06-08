use chrono::DateTime;
use chrono::Utc;
use sqlx::FromRow;
use uuid::Uuid;

use api::responses::logs::LogResponse;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Log {
    pub id: Uuid,
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: DateTime<Utc>,
}

impl From<Log> for LogResponse {
    fn from(log: Log) -> Self {
        LogResponse {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: log.timestamp,
        }
    }
}

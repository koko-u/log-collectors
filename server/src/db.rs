use std::path;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;

use crate::errors::AppError;
use crate::models::logs::Log;

pub mod csv;
pub mod logs;

#[async_trait]
pub trait DbTrait {
    async fn insert_log(
        &self,
        user_agent: &str,
        response_time: i32,
        timestamp: Option<DateTime<Utc>>,
    ) -> error_stack::Result<Log, AppError>;

    async fn get_logs(
        &self,
        from: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> error_stack::Result<Vec<Log>, AppError>;

    async fn load_file<P>(&self, file_path: P) -> error_stack::Result<u64, AppError>
    where
        P: AsRef<path::Path> + Send;
}

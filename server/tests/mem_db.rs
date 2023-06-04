use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::SubsecRound;
use chrono::Utc;
use server::db::DbTrait;
use server::errors::AppError;
use server::models::logs::Log;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct MemDb {
    pub logs: Arc<RwLock<Vec<Log>>>,
}
impl From<Vec<Log>> for MemDb {
    fn from(logs: Vec<Log>) -> Self {
        let db = Self::default();
        {
            let mut writer = db.logs.write().unwrap();
            for log in logs {
                writer.push(log);
            }
        }
        db
    }
}

#[async_trait]
impl DbTrait for MemDb {
    async fn insert_log(
        &self,
        user_agent: &str,
        response_time: i32,
        timestamp: Option<DateTime<Utc>>,
    ) -> error_stack::Result<Log, AppError> {
        let mut logs = self.logs.write().unwrap();
        let log = Log {
            id: Uuid::new_v4(),
            user_agent: user_agent.into(),
            response_time,
            timestamp: timestamp.unwrap_or_else(|| Utc::now().trunc_subsecs(0)),
        };

        logs.push(log.clone());

        Ok(log)
    }

    async fn get_logs(
        &self,
        from: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> error_stack::Result<Vec<Log>, AppError> {
        let logs = self.logs.read().unwrap();

        let logs = logs
            .iter()
            .filter(|log| {
                from.map(|from| log.timestamp >= from).unwrap_or(true)
                    && until.map(|until| log.timestamp <= until).unwrap_or(true)
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(logs)
    }
}

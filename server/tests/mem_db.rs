use std::fs;
use std::io;
use std::path;
use std::sync::RwLock;

use async_trait::async_trait;
use chrono::DateTime;
use chrono::SubsecRound;
use chrono::Utc;
use error_stack::IntoReport;
use error_stack::ResultExt;
use uuid::Uuid;

use server::db::DbTrait;
use server::errors::AppError;
use server::models::logs::Log;

use api::requests::logs::NewLog;

#[derive(Debug, Default)]
pub struct MemDb {
    pub logs: RwLock<Vec<Log>>,
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

    async fn load_file<P>(&self, file_path: P) -> error_stack::Result<u64, AppError>
    where
        P: AsRef<path::Path> + Send,
    {
        let file = fs::File::open(file_path)
            .into_report()
            .change_context(AppError)?;
        let reader = io::BufReader::new(file);
        let new_logs = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader)
            .into_deserialize::<NewLog>();

        let mut count = 0;
        let mut logs = self.logs.write().unwrap();
        for new_log in new_logs {
            let new_log = new_log.into_report().change_context(AppError)?;
            let log = Log {
                id: Uuid::new_v4(),
                user_agent: new_log.user_agent,
                response_time: new_log.response_time,
                timestamp: new_log
                    .timestamp
                    .unwrap_or_else(|| Utc::now().trunc_subsecs(0)),
            };
            logs.push(log);
            count += 1;
        }

        Ok(count)
    }
}

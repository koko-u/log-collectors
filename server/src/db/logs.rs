use chrono::DateTime;
use chrono::Utc;
use error_stack::IntoReport;
use error_stack::ResultExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::logs::Log;

pub async fn insert_log(
    pool: &PgPool,
    user_agent: &str,
    response_time: i32,
) -> error_stack::Result<Log, AppError> {
    let mut conn = pool
        .acquire()
        .await
        .into_report()
        .change_context(AppError)?;

    let id = Uuid::new_v4();

    let created: Log = sqlx::query_as!(
        Log,
        r#"
         INSERT INTO logs (id, user_agent, response_time)
         VALUES ($1, $2, $3)
         RETURNING id, user_agent, response_time, timestamp
         "#,
        id,
        user_agent.to_string(),
        response_time
    )
    .fetch_one(&mut conn)
    .await
    .into_report()
    .change_context(AppError)?;

    Ok(created)
}

pub async fn insert_log_with_timestamp(
    pool: &PgPool,
    user_agent: &str,
    response_time: i32,
    timestamp: DateTime<Utc>,
) -> error_stack::Result<Log, AppError> {
    let mut conn = pool
        .acquire()
        .await
        .into_report()
        .change_context(AppError)?;

    let id = Uuid::new_v4();

    let created: Log = sqlx::query_as!(
        Log,
        r#"
         INSERT INTO logs (id, user_agent, response_time, timestamp)
         VALUES ($1, $2, $3, $4)
         RETURNING id, user_agent, response_time, timestamp
         "#,
        id,
        user_agent.to_string(),
        response_time,
        timestamp
    )
    .fetch_one(&mut conn)
    .await
    .into_report()
    .change_context(AppError)?;

    Ok(created)
}

pub struct LogGetBuilder<'a> {
    pool: &'a PgPool,
    from: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
}
impl<'a> LogGetBuilder<'a> {
    pub fn new<'b: 'a>(pool: &'b PgPool) -> Self {
        Self {
            pool,
            from: None,
            until: None,
        }
    }
    pub async fn execute(&self) -> error_stack::Result<Vec<Log>, AppError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .into_report()
            .change_context(AppError)?;

        let logs: Vec<Log> = sqlx::query_as!(
            Log,
            r#"
                SELECT
                    id,
                    user_agent,
                    response_time,
                    timestamp
                FROM
                    logs
                WHERE
                    timestamp >= COALESCE($1, timestamp)
                    AND
                    timestamp <= COALESCE($2, timestamp)
            "#,
            self.from,
            self.until
        )
        .fetch_all(&mut conn)
        .await
        .into_report()
        .change_context(AppError)?;

        Ok(logs)
    }
    pub fn from(self, from: DateTime<Utc>) -> LogGetBuilder<'a> {
        Self {
            from: Some(from),
            ..self
        }
    }
    pub fn until(self, until: DateTime<Utc>) -> LogGetBuilder<'a> {
        Self {
            until: Some(until),
            ..self
        }
    }
}

/*
           r#"
               SELECT
                   id,
                   user_agent,
                   response_time,
                   timestamp
               FROM
                   logs
               WHERE
                   timestamp >= CASE
                                   WHEN "$1::TIMESTAMP WITH TIME ZONE" IS NULL THEN timestamp
                                   ELSE "$1::TIMESTAMP WITH TIME ZONE"
                                END
                   AND
                   timestamp <= CASE
                                   WHEN "$2::TIMESTAMP WITH TIME ZONE" IS NULL THEN timestamp
                                   ELSE "$2::TIMESTAMP WITH TIME ZONE"
                                END
           "#,

*/
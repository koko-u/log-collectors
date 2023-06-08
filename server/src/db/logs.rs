use std::fs;
use std::io;
use std::path;

use api::requests::logs::NewLog;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::SubsecRound;
use chrono::Utc;
use error_stack::IntoReport;
use error_stack::ResultExt;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::logs::Log;
use crate::states::DbState;

use super::DbTrait;

#[async_trait]
impl DbTrait for DbState {
    async fn insert_log(
        &self,
        user_agent: &str,
        response_time: i32,
        timestamp: Option<DateTime<Utc>>,
    ) -> error_stack::Result<Log, AppError> {
        let mut conn = self
            .acquire()
            .await
            .into_report()
            .change_context(AppError)?;

        let id = Uuid::new_v4();
        let timestamp = timestamp.unwrap_or_else(|| Utc::now().trunc_subsecs(0));

        let new_log = sqlx::query_as!(
            Log,
            r#"
            INSERT INTO logs (id, user_agent, response_time, timestamp)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_agent, response_time, timestamp
            "#,
            id,
            user_agent,
            response_time,
            timestamp
        )
        .fetch_one(&mut conn)
        .await
        .into_report()
        .change_context(AppError)?;

        Ok(new_log)
    }

    async fn get_logs(
        &self,
        from: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> error_stack::Result<Vec<Log>, AppError> {
        let mut conn = self
            .acquire()
            .await
            .into_report()
            .change_context(AppError)?;

        let logs = sqlx::query_as!(
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
            from,
            until
        )
        .fetch_all(&mut conn)
        .await
        .into_report()
        .change_context(AppError)?;

        Ok(logs)
    }

    async fn load_file<P>(&self, file_path: P) -> error_stack::Result<u64, AppError>
    where
        P: AsRef<path::Path> + Send,
    {
        let mut conn = self
            .acquire()
            .await
            .into_report()
            .change_context(AppError)?;

        let mut line_count = 0;

        let file = fs::File::open(file_path)
            .into_report()
            .change_context(AppError)?;
        let reader = io::BufReader::new(file);

        let logs_iter = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader)
            .into_deserialize::<NewLog>();

        let logs_vec = logs_iter.collect::<Vec<_>>();
        log::debug!("logs vec: {logs_vec:?}");

        let chunk_size = 1000;

        // log data
        let mut id_vec = Vec::with_capacity(chunk_size);
        let mut user_agent_vec = Vec::with_capacity(chunk_size);
        let mut response_time_vec = Vec::with_capacity(chunk_size);
        let mut timestamp_vec = Vec::with_capacity(chunk_size);

        for (index, log) in logs_vec.into_iter().enumerate() {
            if log.is_err() {
                // skip error rows
                log::debug!("csv error: {:?}", log.err());
                continue;
            }
            let log = log.unwrap();

            log::debug!("NewLog: {log:?}");

            // cs 10
            // idx 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15
            // rem  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
            //
            //                                 x

            if index % chunk_size == chunk_size - 1 {
                // update logs table
                line_count += bulk_insert_logs(
                    &mut conn,
                    &id_vec,
                    &user_agent_vec,
                    &response_time_vec,
                    &timestamp_vec,
                )
                .await?;

                // clear data
                id_vec.clear();
                user_agent_vec.clear();
                response_time_vec.clear();
                timestamp_vec.clear();
            } else {
                // store log data
                id_vec.push(Uuid::new_v4());
                user_agent_vec.push(log.user_agent);
                response_time_vec.push(log.response_time);
                timestamp_vec.push(log.timestamp.unwrap_or_else(|| Utc::now().trunc_subsecs(0)));
            }
        }

        // upload remaining logs
        if !id_vec.is_empty() {
            line_count += bulk_insert_logs(
                &mut conn,
                &id_vec,
                &user_agent_vec,
                &response_time_vec,
                &timestamp_vec,
            )
            .await?;
        }

        Ok(line_count)
    }
}

async fn bulk_insert_logs(
    conn: &mut PoolConnection<Postgres>,
    id_vec: &[Uuid],
    user_agent_vec: &[String],
    response_time_vec: &[i32],
    timestamp_vec: &[DateTime<Utc>],
) -> error_stack::Result<u64, AppError> {
    let n = sqlx::query!(
                    r#"
                    INSERT INTO logs (
                        id,
                        user_agent,
                        response_time,
                        timestamp
                    )
                    SELECT
                        id,
                        user_agent,
                        response_time,
                        timestamp
                    FROM
                        UNNEST($1::UUID[], $2::TEXT[], $3::INT[], $4::TIMESTAMP WITH TIME ZONE[]) AS a(id, user_agent, response_time, timestamp)
                    "#,
                    id_vec,
                    user_agent_vec,
                    response_time_vec,
                    timestamp_vec
                )
                .execute(conn)
                .await
                .into_report()
                .change_context(AppError)?;

    Ok(n.rows_affected())
}

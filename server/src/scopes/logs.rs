use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::db::logs::insert_log;
use crate::db::logs::insert_log_with_timestamp;
use crate::db::logs::LogGetBuilder;
use crate::errors::AppResponseError;
use crate::states::DbState;

use api::params::DateTimeRange;
use api::requests::logs::NewLog;
use api::responses::logs::LogResponse;

pub fn logs_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/logs")
            .route("", web::post().to(post_logs))
            .route("", web::get().to(get_logs)),
    );
}

async fn post_logs(
    app_state: web::Data<DbState>,
    new_log: web::Json<NewLog>,
) -> Result<impl Responder, AppResponseError> {
    let NewLog {
        user_agent,
        response_time,
        timestamp,
    } = new_log.into_inner();

    let log = match timestamp {
        Some(timestamp) => {
            insert_log_with_timestamp(&app_state, &user_agent, response_time, timestamp).await?
        }
        None => insert_log(&app_state, &user_agent, response_time).await?,
    };

    let response = HttpResponse::Created().json(LogResponse::from(log));

    Ok(response)
}

async fn get_logs(
    app_state: web::Data<DbState>,
    range: web::Query<DateTimeRange>,
) -> Result<impl Responder, AppResponseError> {
    let DateTimeRange { from, until } = range.into_inner();
    let mut log_getter = LogGetBuilder::new(&app_state);
    if let Some(from) = from {
        log_getter = log_getter.from(from);
    }
    if let Some(until) = until {
        log_getter = log_getter.until(until);
    }
    let logs = log_getter.execute().await?;

    let logs = logs.into_iter().map(LogResponse::from).collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(logs))
}

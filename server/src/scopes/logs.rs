use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::db::DbTrait;
use crate::errors::AppResponseError;

use api::params::DateTimeRange;
use api::requests::logs::NewLog;
use api::responses::logs::LogResponse;

pub fn logs_scope<DB: DbTrait + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/logs")
            .route("", web::post().to(post_logs::<DB>))
            .route("", web::get().to(get_logs::<DB>)),
    );
}

async fn post_logs<DB: DbTrait>(
    app_state: web::Data<DB>,
    new_log: web::Json<NewLog>,
) -> Result<impl Responder, AppResponseError> {
    let NewLog {
        user_agent,
        response_time,
        timestamp,
    } = new_log.into_inner();

    let new_log = app_state
        .insert_log(&user_agent, response_time, timestamp)
        .await?;

    let response = HttpResponse::Created().json(LogResponse::from(new_log));

    Ok(response)
}

async fn get_logs<DB: DbTrait>(
    app_state: web::Data<DB>,
    range: web::Query<DateTimeRange>,
) -> Result<impl Responder, AppResponseError> {
    let DateTimeRange { from, until } = range.into_inner();

    let logs = app_state.get_logs(from, until).await?;

    let logs = logs.into_iter().map(LogResponse::from).collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(logs))
}

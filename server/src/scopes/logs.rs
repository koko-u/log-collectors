use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::errors::AppResponseError;
use crate::states::AppState;
use api::params::DateTimeRange;
use api::requests::logs::NewLog;
use api::responses::logs::LogResponse;
use chrono::Utc;

pub fn logs_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/logs")
            .route("", web::post().to(post_logs))
            .route("", web::get().to(get_logs)),
    );
}

async fn post_logs(
    _app_state: web::Data<AppState>,
    new_log: web::Json<NewLog>,
) -> Result<impl Responder, AppResponseError> {
    let new_log = new_log.into_inner();
    log::debug!("new_log request: {new_log:#?}");

    todo::TODO!("POST /logs"; Ok(HttpResponse::Created().finish()))
}

async fn get_logs(
    _app_state: web::Data<AppState>,
    range: web::Query<DateTimeRange>,
) -> Result<impl Responder, AppResponseError> {
    let range = range.into_inner();
    log::debug!("get log query params: {range}");

    let response = vec![LogResponse {
        user_agent: "Agent 1".into(),
        response_time: 100,
        timestamp: Utc::now(),
    }];

    todo::TODO!("GET /logs"; Ok(HttpResponse::Ok().json(response)))
}

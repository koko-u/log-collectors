use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::errors::AppResponseError;

pub fn logs_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/logs")
            .route("", web::post().to(post_logs))
            .route("", web::get().to(get_logs)),
    );
}

async fn post_logs() -> Result<impl Responder, AppResponseError> {
    todo::TODO!("POST /logs"; Ok(HttpResponse::Ok().finish()))
}

async fn get_logs() -> Result<impl Responder, AppResponseError> {
    todo::TODO!("GET /logs"; Ok(HttpResponse::Ok().finish()))
}

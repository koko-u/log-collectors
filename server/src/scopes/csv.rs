use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::errors::AppResponseError;

pub fn csv_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/csv")
            .route("", web::post().to(post_csv))
            .route("", web::get().to(get_csv)),
    );
}

async fn post_csv() -> Result<impl Responder, AppResponseError> {
    todo::TODO!("POST /csv"; Ok(HttpResponse::Ok().finish()))
}

async fn get_csv() -> Result<impl Responder, AppResponseError> {
    todo::TODO!("GET /csv"; Ok(HttpResponse::Ok().finish()))
}

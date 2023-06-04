use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::db::DbTrait;
use crate::errors::AppResponseError;
use actix_web::http;
use api::responses::csv::CsvResponse;

pub fn csv_scope<DB: DbTrait + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/csv")
            .route("", web::post().to(post_csv::<DB>))
            .route("", web::get().to(get_csv::<DB>)),
    );
}

async fn post_csv<DB: DbTrait>(
    _app_state: web::Data<DB>,
) -> Result<impl Responder, AppResponseError> {
    let response = CsvResponse::default();

    todo::TODO!("POST /csv"; Ok(HttpResponse::Ok().json(response)))
}

async fn get_csv<DB: DbTrait>(
    _app_state: web::Data<DB>,
) -> Result<impl Responder, AppResponseError> {
    let csv = vec![42_u8, 18_u8];

    todo::TODO!("GET /csv"; Ok(HttpResponse::Ok().insert_header((http::header::CONTENT_TYPE, "text/csv")).body(csv)))
}

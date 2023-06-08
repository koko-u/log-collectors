use std::io::Write;

use crate::db::DbTrait;
use crate::errors::AppError;
use crate::errors::AppResponseError;
use actix_multipart::Multipart;
use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use api::responses::csv::CsvResponse;
use error_stack::IntoReport;
use error_stack::ResultExt;
use futures_util::StreamExt;

pub fn csv_scope<DB: DbTrait + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/csv")
            .route("", web::post().to(post_csv::<DB>))
            .route("", web::get().to(get_csv::<DB>)),
    );
}

async fn post_csv<DB: DbTrait>(
    app_state: web::Data<DB>,
    mut multi_part: Multipart,
) -> Result<impl Responder, AppResponseError> {
    let mut line_count = 0;

    while let Some(field) = multi_part.next().await {
        let mut field = field?;

        log::debug!(
            "Name: {}, Content-Disposition: {}, Content-Type: {:?}",
            field.name(),
            field.content_disposition(),
            field.content_type()
        );

        if field
            .content_type()
            .is_some_and(|content_type| *content_type == mime::TEXT_CSV)
        {
            let mut tmpfile = tempfile::Builder::new()
                .append(true)
                .tempfile()
                .into_report()
                .change_context(AppError)?;

            log::debug!("tmpfile: {tmpfile:?}");

            while let Some(bytes) = field.next().await {
                let bytes = bytes?;

                log::debug!("{bytes:?}");

                tmpfile
                    .write_all(&bytes)
                    .into_report()
                    .change_context(AppError)?;
            }
            tmpfile.flush().into_report().change_context(AppError)?;
            line_count += app_state.load_file(tmpfile.path()).await?;
        }
    }

    let response = CsvResponse(line_count);
    Ok(HttpResponse::Ok().json(response))
}

async fn get_csv<DB: DbTrait>(
    _app_state: web::Data<DB>,
) -> Result<impl Responder, AppResponseError> {
    let csv = vec![42_u8, 18_u8];

    todo::TODO!("GET /csv"; Ok(HttpResponse::Ok().insert_header((http::header::CONTENT_TYPE, "text/csv")).body(csv)))
}

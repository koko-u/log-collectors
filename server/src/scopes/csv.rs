use std::io::Write;

use crate::db::DbTrait;
use crate::errors::AppError;
use crate::errors::AppResponseError;
use actix_multipart::Multipart;
use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use api::params::DateTimeRange;
use api::responses::csv::CsvResponse;
use api::responses::logs::LogResponse;
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

        if field
            .content_type()
            .is_some_and(|content_type| *content_type == mime::TEXT_CSV)
        {
            let mut tmpfile = tempfile::Builder::new()
                .append(true)
                .tempfile()
                .into_report()
                .change_context(AppError)?;

            while let Some(bytes) = field.next().await {
                let bytes = bytes?;

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
    app_state: web::Data<DB>,
    range: web::Query<DateTimeRange>,
) -> Result<impl Responder, AppResponseError> {
    let DateTimeRange { from, until } = range.into_inner();

    let logs = app_state.get_logs(from, until).await?;

    let v = Vec::new();
    let mut w = csv::WriterBuilder::new().has_headers(false).from_writer(v);

    for log in logs {
        let log_response = LogResponse {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: log.timestamp,
        };
        w.serialize(log_response)
            .into_report()
            .change_context(AppError)?;
    }

    let csv = w.into_inner().into_report().change_context(AppError)?;
    let response = HttpResponse::Ok()
        .insert_header((http::header::CONTENT_TYPE, mime::TEXT_CSV_UTF_8))
        .body(csv);

    Ok(response)
}

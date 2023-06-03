use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use error_stack::{IntoReport, ResultExt};
use server::{
    errors::AppError,
    scopes::{csv::csv_scope, logs::logs_scope},
    states::AppState,
};
use std::net;

#[actix_web::main]
async fn main() -> error_stack::Result<(), AppError> {
    dotenv::dotenv().into_report().change_context(AppError)?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Listening on {addr:?}");

    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(app_state.clone())
            .configure(csv_scope)
            .configure(logs_scope)
    })
    .bind(addr)
    .into_report()
    .change_context(AppError)?
    .run()
    .await
    .into_report()
    .change_context(AppError)?;

    Ok(())
}

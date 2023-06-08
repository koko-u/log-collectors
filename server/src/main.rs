use std::env;
use std::net;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use env_logger::Env;
use error_stack::IntoReport;
use error_stack::ResultExt;

use server::errors::AppError;
use server::scopes::csv::csv_scope;
use server::scopes::logs::logs_scope;
use server::states::DbState;

#[actix_web::main]
async fn main() -> error_stack::Result<(), AppError> {
    dotenv::dotenv().into_report().change_context(AppError)?;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Listening on {addr:?}");

    let database_url = env::var("DATABASE_URL")
        .into_report()
        .change_context(AppError)?;
    let db_state = DbState::new(&database_url).await?;
    let app_state = web::Data::new(db_state);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(middleware::Compress::default())
            .app_data(app_state.clone())
            .configure(csv_scope::<DbState>)
            .configure(logs_scope::<DbState>)
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

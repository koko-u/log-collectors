use std::env;

use actix_web::http;
use actix_web::test;
use actix_web::web;
use actix_web::App;
use api::requests::logs::NewLog;
use pretty_assertions::assert_eq;
use server::scopes::csv::csv_scope;
use server::scopes::logs::logs_scope;
use server::states::DbState;

async fn get_db_state() -> DbState {
    dotenv::dotenv().unwrap();
    let url = env::var("DATABASE_URL").unwrap();
    DbState::new(&url).await.unwrap()
}

#[actix_web::test]
async fn ping_get_logs() {
    let app_state = web::Data::new(get_db_state().await);
    let app = test::init_service(App::new().app_data(app_state).configure(logs_scope)).await;
    let req = test::TestRequest::get().uri("/logs").to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn ping_post_logs() {
    let app_state = web::Data::new(get_db_state().await);
    let app = test::init_service(App::new().app_data(app_state).configure(logs_scope)).await;
    let req = test::TestRequest::post()
        .uri("/logs")
        .append_header(http::header::ContentType::json())
        .set_json(NewLog {
            user_agent: "Agent 1".into(),
            response_time: 100,
            timestamp: None,
        })
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::CREATED);
}

#[actix_web::test]
async fn ping_get_csv() {
    let app_state = web::Data::new(get_db_state().await);
    let app = test::init_service(App::new().app_data(app_state).configure(csv_scope)).await;
    let req = test::TestRequest::get().uri("/csv").to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn ping_post_csv() {
    let app_state = web::Data::new(get_db_state().await);
    let app = test::init_service(App::new().app_data(app_state).configure(csv_scope)).await;
    let req = test::TestRequest::post()
        .uri("/csv")
        .append_header(http::header::ContentType::json())
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);
}

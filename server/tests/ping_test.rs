use actix_web::http;
use actix_web::test;
use actix_web::App;
use pretty_assertions::assert_eq;
use server::scopes::csv::csv_scope;
use server::scopes::logs::logs_scope;

#[actix_web::test]
async fn ping_get_logs() {
    let app = test::init_service(App::new().configure(logs_scope)).await;
    let req = test::TestRequest::get().uri("/logs").to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn ping_post_logs() {
    let app = test::init_service(App::new().configure(logs_scope)).await;
    let req = test::TestRequest::post()
        .uri("/logs")
        .append_header(http::header::ContentType::json())
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn ping_get_csv() {
    let app = test::init_service(App::new().configure(csv_scope)).await;
    let req = test::TestRequest::get().uri("/csv").to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn ping_post_csv() {
    let app = test::init_service(App::new().configure(csv_scope)).await;
    let req = test::TestRequest::post()
        .uri("/csv")
        .append_header(http::header::ContentType::json())
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), http::StatusCode::OK);
}

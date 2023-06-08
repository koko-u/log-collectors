use actix_web::http;
use actix_web::test;
use actix_web::web;
use actix_web::App;
use chrono::SubsecRound;
use chrono::Utc;
use pretty_assertions::assert_eq;
use uuid::Uuid;

use server::models::logs::Log;
use server::scopes::logs::logs_scope;

use api::requests::logs::NewLog;
use api::responses::logs::LogResponse;

mod mem_db;

#[actix_web::test]
async fn create_logs() {
    let mem_db = mem_db::MemDb::default();
    let app_state = web::Data::new(mem_db);

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(logs_scope::<mem_db::MemDb>),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/logs")
        .append_header(http::header::ContentType::json())
        .set_json(NewLog {
            user_agent: "Agent 1".into(),
            response_time: 100,
            timestamp: None,
        })
        .to_request();
    let res: LogResponse = test::call_and_read_body_json(&app, req).await;

    assert_eq!(res.user_agent, "Agent 1");
}

#[actix_web::test]
async fn get_logs() {
    let log1 = Log {
        id: Uuid::new_v4(),
        user_agent: "agent 1".into(),
        response_time: 100,
        timestamp: Utc::now().trunc_subsecs(0),
    };
    let log2 = Log {
        id: Uuid::new_v4(),
        user_agent: "agent 2".into(),
        response_time: 200,
        timestamp: Utc::now().trunc_subsecs(0),
    };

    let mem_db = mem_db::MemDb::from(vec![log1.clone(), log2.clone()]);
    let app_state = web::Data::new(mem_db);

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(logs_scope::<mem_db::MemDb>),
    )
    .await;
    let req = actix_web::test::TestRequest::get()
        .uri("/logs")
        .to_request();
    let res: Vec<LogResponse> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(res, vec![LogResponse::from(log1), LogResponse::from(log2)]);
}

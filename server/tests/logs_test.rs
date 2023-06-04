use std::env;

use actix_web::http;
use actix_web::rt::Runtime;
use actix_web::web;
use actix_web::App;
use api::requests::logs::NewLog;
use api::responses::logs::LogResponse;
use chrono::SubsecRound;
use chrono::Utc;
use pretty_assertions::assert_eq;
use server::models::logs::Log;
use server::scopes::logs::logs_scope;
use server::states::DbState;
use sqlx::PgPool;
use uuid::Uuid;

struct Fixture<'a> {
    runtime: &'a Runtime,
    db_state: web::Data<DbState>,
}
impl<'a> Fixture<'a> {
    fn new<'b: 'a>(r: &'b Runtime) -> Self {
        dotenv::dotenv().expect("dotenv");
        //env_logger::init();

        let url = env::var("DATABASE_URL").expect("env::var(DATABASE_URL)");
        let pool = r
            .block_on(PgPool::connect(&url))
            .expect("PgPool::connect(url)");

        let mut conn = r.block_on(pool.acquire()).expect("acquire connection");
        r.block_on(sqlx::query!("DELETE FROM logs").execute(&mut conn))
            .expect("delete logs");

        Self {
            runtime: r,
            db_state: web::Data::new(pool.into()),
        }
    }
    fn setup_logs(&self, logs: &[Log]) {
        let mut conn = self
            .runtime
            .block_on(self.db_state.acquire())
            .expect("aquire connection");

        self.runtime
            .block_on(sqlx::query!("DELETE FROM logs").execute(&mut conn))
            .expect("delete logs");

        for log in logs {
            self.runtime
                .block_on(
                    sqlx::query!(
                        r#"
                        INSERT INTO logs (id, user_agent, response_time, timestamp)
                        VALUES ($1, $2, $3, $4)
                        "#,
                        log.id,
                        log.user_agent,
                        log.response_time,
                        log.timestamp
                    )
                    .execute(&mut conn),
                )
                .expect("insert log");
        }
    }
}
impl<'a> Drop for Fixture<'a> {
    fn drop(&mut self) {
        let mut conn = self
            .runtime
            .block_on(self.db_state.acquire())
            .expect("db_state.acquire()");
        self.runtime
            .block_on(sqlx::query!("DELETE FROM logs").execute(&mut conn))
            .expect("DELETE TABLE logs");
    }
}

#[test]
fn create_logs() {
    let r = actix_web::rt::Runtime::new().expect("Runtime::new()");
    let f = Fixture::new(&r);

    let app = f.runtime.block_on(actix_web::test::init_service(
        App::new()
            .app_data(f.db_state.clone())
            .configure(logs_scope),
    ));
    let req = actix_web::test::TestRequest::post()
        .uri("/logs")
        .append_header(http::header::ContentType::json())
        .set_json(NewLog {
            user_agent: "Agent 1".into(),
            response_time: 100,
            timestamp: None,
        })
        .to_request();
    let res: LogResponse = f
        .runtime
        .block_on(actix_web::test::call_and_read_body_json(&app, req));

    assert_eq!(res.user_agent, "Agent 1");
}

#[test]
fn get_logs() {
    let r = actix_web::rt::Runtime::new().expect("Runtime::new()");
    let f = Fixture::new(&r);

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

    f.setup_logs(&[log1.clone(), log2.clone()]);

    let app = f.runtime.block_on(actix_web::test::init_service(
        App::new()
            .app_data(f.db_state.clone())
            .configure(logs_scope),
    ));
    let req = actix_web::test::TestRequest::get()
        .uri("/logs")
        .to_request();
    let res: Vec<LogResponse> = f
        .runtime
        .block_on(actix_web::test::call_and_read_body_json(&app, req));

    assert_eq!(res, vec![LogResponse::from(log1), LogResponse::from(log2)]);
}

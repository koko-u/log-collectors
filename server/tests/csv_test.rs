use actix_web::http;
use actix_web::test;
use actix_web::web;
use actix_web::App;
use server::scopes::csv::csv_scope;

mod mem_db;

#[actix_web::test]
async fn post_csv() {
    let mem_db = mem_db::MemDb::default();
    let app_state = web::Data::new(mem_db);

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(csv_scope::<mem_db::MemDb>),
    )
    .await;

    let bytes = web::Bytes::from(
        "\r\n\
        ------WebKitFormBoundary7MA4YWxkTrZu0gW\r\n\
        Content-Disposition: form-data; name=\"file\"; filename=\"a.csv\"\r\n\
        Content-Type: text/csv\r\n
        \r\n\
        \"agent a\", 100, 2023-01-02 03:04:07.682066134 UTC\r\n\
        \"agent b\", 200, 2023-02-03 04:05:09.721651021 UTC\r\n\
        ------WebKitFormBoundary7MA4YWxkTrZu0gW--\r\n",
    );
    let header = (
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static(
            r#"multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW"#,
        ),
    );

    let req = test::TestRequest::post()
        .uri("/csv")
        .append_header(header)
        .set_payload(bytes)
        .to_request();
    let res_body = test::call_and_read_body(&app, req).await;
    let res_str = String::from_utf8(res_body.to_vec()).unwrap();

    assert_eq!(res_str, "2");
}

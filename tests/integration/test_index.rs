use crate::fixtures;
use actix_web::{http::header::ContentType, test, web, App};
use leaflet_rounds::routes::index;

#[actix_web::test]
async fn test_index() {
    let app = test::init_service(App::new().route("/", web::get().to(index))).await;

    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_and_read_body(&app, req).await;

    let page_str = std::str::from_utf8(resp.as_ref()).unwrap();

    let page = fixtures::get_page_element(page_str, "h1");

    assert_eq!(page, "Hello world!")
}

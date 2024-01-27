use crate::fixtures;
use actix_web::{test, web, App};
use leaflet_rounds::routes::{index, not_found};

#[actix_web::test]
async fn test_page_not_found_title() {
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(index))
            .default_service(web::get().to(not_found)),
    )
    .await;

    let req = test::TestRequest::get().uri("/foo").to_request();

    let resp = test::call_and_read_body(&app, req).await;

    let page_str = std::str::from_utf8(resp.as_ref()).unwrap();

    let page = fixtures::get_page_element(page_str, "h1");

    assert_eq!(page, "Page not found");
}

#[actix_web::test]
async fn test_page_not_found_status() {
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(index))
            .default_service(web::get().to(not_found)),
    )
    .await;

    let req = test::TestRequest::get().uri("/foo").to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404)
}

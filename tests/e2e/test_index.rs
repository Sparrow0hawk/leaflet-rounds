use crate::fixtures;
use fantoccini::wd::Capabilities;
use fantoccini::{ClientBuilder, Locator};
use serde_json::json;

#[tokio::test]
async fn test_index_shows_hello_world() {
    let app = fixtures::spawn_app().await;

    let mut caps = Capabilities::new();
    caps.insert(
        String::from("moz:firefoxOptions"),
        json!({"args": ["--headless"]}),
    );

    let driver = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:4444")
        .await
        .expect("Web driver failed to start");

    driver
        .goto(&app.address)
        .await
        .expect("Failed to load home page");

    let header = driver.find(Locator::Css("h1")).await.unwrap();

    assert!(header.text().await.unwrap() == "Map my round");

    driver.close().await.unwrap();
}

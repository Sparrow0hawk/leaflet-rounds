use crate::fixtures;
use thirtyfour::prelude::*;

async fn find_tag_text(driver: &WebDriver, tag_name: &str) -> WebDriverResult<String> {
    let tag = driver.find(By::Tag(tag_name)).await?;
    tag.text().await
}

#[tokio::test]
async fn test_index_shows_hello_world() -> WebDriverResult<()> {
    let app = fixtures::spawn_app().await;

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", caps)
        .await?;

    driver
        .goto(&app.address)
        .await?;

    let header_text = find_tag_text(&driver, "h1").await;

    driver.quit().await?;

    assert_eq!(header_text?, "Map my round");

    Ok(())
}

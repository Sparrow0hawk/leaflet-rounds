use crate::fixtures;
use thirtyfour::prelude::*;

async fn find_element(driver: &WebDriver, by: By) -> WebDriverResult<WebElement> {
    driver.find(by).await
}

async fn get_text(driver: &WebDriver, by: By) -> WebDriverResult<String> {
    let element = find_element(&driver, by).await?;

    element.text().await
}

async fn get_clickable(driver: &WebDriver, by: By) -> WebDriverResult<bool> {
    let element = find_element(&driver, by).await?;

    element.is_clickable().await
}

#[tokio::test]
async fn test_index_shows_hello_world() -> WebDriverResult<()> {
    let app = fixtures::spawn_app().await;

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto(&app.address).await?;

    let by = By::Tag("h1");

    let header_text = get_text(&driver, by).await;

    driver.quit().await?;

    assert_eq!(header_text?, "Map my round");

    Ok(())
}

#[tokio::test]
async fn test_index_shows_map() -> WebDriverResult<()> {
    let app = fixtures::spawn_app().await;

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto(&app.address).await?;

    let by = By::ClassName("leaflet-map-pane");

    let map = get_clickable(&driver, by).await;

    driver.quit().await?;

    assert!(map?);

    Ok(())
}

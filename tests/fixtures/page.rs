use scraper::Selector;

pub fn get_page_element(body: &str, element: &str) -> String {
    let fragment = scraper::Html::parse_document(body);

    let selector = Selector::parse(element).unwrap();

    fragment.select(&selector).next().unwrap().inner_html()
}

pub struct NotFoundPage {
    pub page: String,
    pub content: String,
}

impl NotFoundPage {
    pub fn new(page: &str) -> Self {
        NotFoundPage {
            page: String::from(page),
            content: get_page_element(&page, "div.row"),
        }
    }
    pub fn heading(self) -> String {
        get_page_element(&self.content, "h1")
    }
}

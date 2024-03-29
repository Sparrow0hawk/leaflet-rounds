use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

pub async fn index() -> impl Responder {
    let template = IndexTemplate {
        title: "Map my round",
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

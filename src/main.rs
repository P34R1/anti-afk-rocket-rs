#[macro_use]
extern crate rocket;
use askama::Template;

use rocket::{fs::NamedFile, response::content::RawHtml};
use std::path::Path;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    text: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    let title = String::from("Index");
    let text = String::from("Hello, from Rust!");
    let rendered_template = IndexTemplate { title, text }
        .render()
        .expect("template rendering");
    RawHtml(rendered_template)
}

#[get("/index.js")]
async fn js() -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/index.js")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, js])
}

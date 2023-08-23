use askama::Template;
use rocket::response::content::RawHtml;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    text: String,
}

#[get("/")]
pub fn index() -> RawHtml<String> {
    let title = String::from("Index");
    let text = String::from("Hello, from Rust!");
    let rendered_template = IndexTemplate { title, text }
        .render()
        .expect("template rendering");
    RawHtml(rendered_template)
}

#[get("/favicon.ico")]
pub fn favicon() -> &'static [u8] {
    include_bytes!("../favicon.ico")
}

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![index, favicon]
}

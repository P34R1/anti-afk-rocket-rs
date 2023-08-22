#[macro_use]
extern crate rocket;
use askama::Template;
use rocket::response::content::RawHtml;

mod api;

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

#[get("/favicon.ico")]
fn favicon() -> &'static [u8] {
    include_bytes!("../favicon.ico")
}

mod key_handler;

#[launch]
fn rocket() -> _ {
    let key_handler_state = key_handler::KeyHandler;

    rocket::build()
        .manage(key_handler_state)
        .mount("/", routes![index, favicon])
        .mount("/api", api::get_routes())
}

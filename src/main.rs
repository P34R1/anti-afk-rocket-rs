#[macro_use]
extern crate rocket;

mod api;

use rocket::response::content::RawCss;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let title = String::from("Index");
    let text = String::from("Hello, from Rust!");
    Template::render("index-raw", context! { title, text })
}

#[get("/favicon.ico")]
fn icon() -> &'static [u8] {
    include_bytes!("../static/favicon.ico")
}

#[get("/style.css")]
fn css() -> RawCss<&'static str> {
    RawCss(include_str!("../static/style.css"))
}

#[launch]
fn rocket() -> _ {
    let key_handler_state = api::key_handler::KeyHandler::new();

    rocket::build()
        .manage(key_handler_state)
        .mount("/", routes![index])
        .mount("/static", routes![icon, css])
        .mount("/api", api::get_routes())
        .attach(Template::fairing())
}

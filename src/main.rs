#[macro_use]
extern crate rocket;

mod api;
mod setup;

use rocket::response::content::RawHtml;
use setup::{figments, static_files, templates};

use api::key_types::KeyState;

use rocket_dyn_templates::{context, Template};
use std::sync::{Arc, RwLock};

#[get("/")]
fn index() -> RawHtml<Template> {
    let title = String::from("Index");
    let text = String::from("Hello, from Rust!");
    RawHtml(Template::render("index", context! { title, text }))
}

#[launch]
fn rocket() -> _ {
    let figment = figments::get_figment();
    let key_state = Arc::new(RwLock::new(KeyState::Idle));

    let rocket_build = rocket::custom(figment)
        .manage(key_state)
        .mount("/", routes![index])
        .mount("/static", static_files::get_routes())
        .mount("/api", api::get_routes());

    templates::setup_templates(rocket_build)
}

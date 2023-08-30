#[macro_use]
extern crate rocket;

mod api;
mod setup;

use api::key_types::KeyState;
use setup::{figments, static_files, templates};

use rocket::response::content::RawHtml;

use rocket_dyn_templates::{context, Template};
use std::sync::{Arc, RwLock};

#[get("/")]
fn index() -> RawHtml<Template> {
    let title = String::from("Anti Afk");
    let text = String::from("Anti Afk Utility!");
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

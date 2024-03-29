#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rocket;

mod api;
mod setup;

use api::{get_api_routes, KeyState};
use setup::{get_static_routes, templates};

use rocket::{response::content::RawHtml, Config};

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
    let figment = if cfg!(debug_assertions) {
        Config::figment()
    } else {
        Config::figment().merge(("template_dir", "."))
    };

    let key_state = Arc::new(RwLock::new(KeyState::Idle));

    #[cfg(not(debug_assertions))]
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(2));
        webbrowser::open("http://127.0.0.1:8000").expect("open browser");
    });

    rocket::custom(figment)
        .manage(key_state)
        .mount("/", routes![index])
        .mount("/static", get_static_routes())
        .mount("/api", get_api_routes())
        .attach(Template::custom(templates))
}

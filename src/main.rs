#[macro_use]
extern crate rocket;

mod api;
mod static_files;

use api::key_types::KeyState;
use rocket_dyn_templates::{context, Template};
use std::sync::{Arc, RwLock};

#[get("/")]
fn index() -> Template {
    let title = String::from("Index");
    let text = String::from("Hello, from Rust!");
    Template::render("index", context! { title, text })
}

#[launch]
fn rocket() -> _ {
    let key_state = Arc::new(RwLock::new(KeyState::Idle));

    rocket::build()
        .manage(key_state)
        .mount("/", routes![index])
        .mount("/static", static_files::get_routes())
        .mount("/api", api::get_routes())
        .attach(Template::fairing())
}

#[macro_use]
extern crate rocket;

mod api;
mod static_files;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let title = String::from("Index");
    let text = String::from("Hello, from Rust!");
    Template::render("index", context! { title, text })
}

#[launch]
fn rocket() -> _ {
    let key_handler_state = api::key_handler::KeyHandler::new();

    rocket::build()
        .manage(key_handler_state)
        .mount("/", routes![index])
        .mount("/static", static_files::get_routes())
        .mount("/api", api::get_routes())
        .attach(Template::fairing())
}

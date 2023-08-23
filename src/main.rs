#[macro_use]
extern crate rocket;

mod main_page;

mod api;
mod key_handler;

#[launch]
fn rocket() -> _ {
    let key_handler_state = key_handler::KeyHandler::new();

    rocket::build()
        .manage(key_handler_state)
        .mount("/", main_page::get_routes())
        .mount("/api", api::get_routes())
}

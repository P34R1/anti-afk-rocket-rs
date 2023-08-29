pub(crate) mod key_handler;
mod repeatkey;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![repeatkey::start_repeating, repeatkey::stop_repeating]
}

pub(crate) mod key_handler;
pub mod presskey;
pub mod repeatkey;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![
        presskey::press_key,
        repeatkey::start_repeating,
        repeatkey::stop_repeating
    ]
}

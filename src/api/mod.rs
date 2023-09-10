#[derive(Debug, PartialEq)]
pub enum KeyState {
    Idle,
    Repeating(enigo::Key, u64),
}

mod close;
mod repeatkey;

pub fn get_api_routes() -> std::vec::Vec<rocket::Route> {
    routes![
        repeatkey::start_repeating,
        repeatkey::stop_repeating,
        close::shutdown,
    ]
}

pub(crate) mod key_types {
    #[derive(Debug, PartialEq)]
    pub enum KeyState {
        Idle,
        Repeating(enigo::Key, u64),
    }
}

mod repeatkey;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![repeatkey::start_repeating, repeatkey::stop_repeating]
}

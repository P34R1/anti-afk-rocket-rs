pub(crate) mod repeatkey;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![repeatkey::repeat_key, repeatkey::stop_repeating]
}

mod presskey;

pub fn get_routes() -> std::vec::Vec<rocket::Route> {
    routes![presskey::press_key]
}

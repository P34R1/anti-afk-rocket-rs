use rocket::response::content::{RawCss, RawJavaScript};

#[get("/favicon.ico")]
pub fn icon() -> &'static [u8] {
    include_bytes!("../static/favicon.ico")
}

#[get("/style.css")]
pub fn css() -> RawCss<&'static str> {
    RawCss(include_str!("../static/style.css"))
}

#[get("/htmx.min.js")]
pub fn js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../static/htmx.min.js"))
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![icon, css, js]
}

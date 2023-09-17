use rocket::response::content::{RawCss, RawJavaScript};

macro_rules! include_static {
    ($file:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file))
    };
}

#[get("/favicon.ico")]
fn icon() -> &'static [u8] {
    include_static!("favicon.ico")
}

#[get("/style.css")]
fn css() -> RawCss<&'static [u8]> {
    RawCss(include_static!("style.css"))
}

#[get("/htmx.min.js")]
fn js() -> RawJavaScript<&'static [u8]> {
    RawJavaScript(include_static!("htmx.min.js"))
}

pub fn get_static_routes() -> Vec<rocket::Route> {
    routes![icon, css, js]
}

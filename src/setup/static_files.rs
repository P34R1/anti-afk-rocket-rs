use rocket::response::content::{RawCss, RawJavaScript};

macro_rules! include_static {
    ($file:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/", $file))
    };

    ($file:expr, $raw_type:ident) => {
        $raw_type(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/",
            $file
        )))
    };
}

#[get("/favicon.ico")]
fn icon() -> &'static [u8] {
    include_static!("favicon.ico")
}

#[get("/style.css")]
fn css() -> RawCss<&'static str> {
    include_static!("style.css", RawCss)
}

#[get("/htmx.min.js")]
fn js() -> RawJavaScript<&'static str> {
    include_static!("htmx.min.js", RawJavaScript)
}

pub fn get_static_routes() -> Vec<rocket::Route> {
    routes![icon, css, js]
}

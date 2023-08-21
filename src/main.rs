#[macro_use]
extern crate rocket;
use askama::Template;

use rocket::response::content::{RawHtml, RawJavaScript};
use rocket::serde::{json::Json, Serialize};

use enigo::{Enigo, Key, KeyboardControllable};
use std::{thread, time::Duration};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    text: String,
}

#[get("/")]
fn index() -> RawHtml<String> {
    let title = String::from("Index");
    let text = String::from("Hello, from Rust!");
    let rendered_template = IndexTemplate { title, text }
        .render()
        .expect("template rendering");
    RawHtml(rendered_template)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct KeyPress {
    pressed_key: Option<String>,
}

#[get("/press/<text>")]
fn press_key(text: String) -> Json<KeyPress> {
    if let Some(key) = text.chars().next() {
        let mut enigo = Enigo::new();

        thread::sleep(Duration::from_secs(1));
        enigo.key_click(Key::Layout(key));

        let pressed_key = Some(String::from(key));
        Json(KeyPress { pressed_key })
    } else {
        let pressed_key = None;
        Json(KeyPress { pressed_key })
    }
}

#[get("/index.js")]
async fn js() -> RawJavaScript<&'static str> {
    RawJavaScript(include_str!("../templates/index.js"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, js])
        .mount("/api", routes![press_key])
}

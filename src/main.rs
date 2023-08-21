#[macro_use]
extern crate rocket;
use askama::Template;

use rocket::{
    response::{content::RawHtml, status::NotFound},
    serde::{json::Json, Serialize},
};

use enigo::{Enigo, KeyboardControllable};
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
    pressed_keys: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct FailedKeyPress {
    err_text: String,
}

#[get("/press/<text>")]
fn press_key(text: String) -> Result<Json<KeyPress>, NotFound<Json<FailedKeyPress>>> {
    if text.is_empty() {
        let err_text = String::from("no key given");
        Err(NotFound(Json(FailedKeyPress { err_text })))
    } else {
        let mut enigo = Enigo::new();

        thread::sleep(Duration::from_secs(1));
        enigo.key_sequence(&text);

        Ok(Json(KeyPress { pressed_keys: text }))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![press_key])
}

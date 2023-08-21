#[macro_use]
extern crate rocket;
use askama::Template;

use rocket::{fs::NamedFile, response::content::RawHtml};

use enigo::{Enigo, Key, KeyboardControllable};
use std::{path::Path, thread, time::Duration};

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

#[get("/press/<text>")]
fn press_key(text: String) -> Option<String> {
    let key = text.chars().next()?;
    let mut enigo = Enigo::new();

    thread::sleep(Duration::from_secs(1));
    enigo.key_click(Key::Layout(key));

    Some(String::from(key))
}

#[get("/index.js")]
async fn js() -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/index.js")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, js])
        .mount("/api", routes![press_key])
}

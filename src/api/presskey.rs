use rocket::{
    response::status::NotFound,
    serde::{json::Json, Serialize},
};

use enigo::{Enigo, KeyboardControllable};
use std::{thread, time::Duration};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct KeyPress {
    pressed_keys: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FailedKeyPress {
    err_text: String,
}

#[get("/press/<text>")]
pub fn press_key(text: String) -> Result<Json<KeyPress>, NotFound<Json<FailedKeyPress>>> {
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

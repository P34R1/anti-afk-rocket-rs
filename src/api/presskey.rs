use rocket::{
    // response::status::NotFound,
    // json::Json,
    serde::Serialize,
    State,
};

use crate::api::key_handler::KeyHandler;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct KeyPress {
    pub(crate) pressed_keys: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FailedKeyPress {
    pub(crate) err_text: String,
}

#[derive(FromForm)]
pub struct Press<'r> {
    #[field(validate = len(1..=1))]
    letter: &'r str,
}

#[get("/press?<query..>")]
pub fn press_key(key_handler: &State<KeyHandler>, query: Press)
// -> Result<Json<KeyPress>, NotFound<Json<FailedKeyPress>>>
{
    let letter = query.letter.chars().next().expect("Next Character");
    let key = enigo::Key::Layout(letter);
    key_handler.press_key(key)
}

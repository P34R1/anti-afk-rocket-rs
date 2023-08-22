use rocket::{
    response::status::NotFound,
    serde::{json::Json, Serialize},
    State,
};

use crate::key_handler::KeyHandler;

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

#[get("/press/<text>")]
pub fn press_key(
    key_handler: &State<KeyHandler>,
    text: String,
) -> Result<Json<KeyPress>, NotFound<Json<FailedKeyPress>>> {
    key_handler
        .press_keys(text)
        .map(Json)
        .map_err(Json)
        .map_err(NotFound)
}

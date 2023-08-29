use rocket::{response::content::RawHtml, State};

use crate::api::key_handler::KeyHandler;

#[derive(FromForm)]
pub struct Repeat<'r> {
    #[field(validate = len(1..=1))]
    letter: &'r str,
    interval_seconds: u64,
}

#[get("/repeat/start?<query..>")]
pub fn start_repeating(key_handler: &State<KeyHandler>, query: Repeat) -> RawHtml<&'static str> {
    key_handler.start_repeating(
        enigo::Key::Layout(query.letter.chars().next().unwrap()),
        query.interval_seconds,
    );

    RawHtml("<p>Successfully Started Repeating</p>")
}

#[get("/repeat/stop")]
pub fn stop_repeating(key_handler: &State<KeyHandler>) -> RawHtml<&'static str> {
    key_handler.stop_repeating();

    RawHtml("<p>Successfully Stopped Repeating</p>")
}

use rocket::State;

use crate::api::key_handler::KeyHandler;

#[derive(FromForm)]
pub struct Repeat<'r> {
    #[field(validate = len(1..=1))]
    letter: &'r str,
    interval_seconds: u64,
}

#[get("/repeat/start?<query..>")]
pub fn repeat_key(key_handler: &State<KeyHandler>, query: Repeat) -> String {
    key_handler.start_repeating(
        enigo::Key::Layout(query.letter.chars().next().unwrap()),
        query.interval_seconds,
    );
    String::from("Successfully Started Repeating")
}

#[get("/repeat/stop")]
pub fn stop_repeating(key_handler: &State<KeyHandler>) -> String {
    key_handler.stop_repeating();
    String::from("Successfully Stopped Repeating")
}

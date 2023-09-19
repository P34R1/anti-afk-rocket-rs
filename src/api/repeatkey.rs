use enigo::{Enigo, KeyboardControllable};
use rocket::{http::Status, response::status::Custom, State};

use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use super::KeyState;

#[derive(FromForm)]
pub struct Repeat<'r> {
    #[field(validate = len(0..=1))]
    letter: &'r str,
    interval_seconds: u64,
}

#[get("/repeat/start?<query..>")]
pub fn start_repeating(
    key_state: &State<Arc<RwLock<KeyState>>>,
    query: Repeat,
) -> Result<&'static str, Custom<&'static str>> {
    let (key, interval_seconds) = (
        query
            .letter
            .chars()
            .next()
            .map(enigo::Key::Layout)
            .ok_or(Custom(Status::UnprocessableEntity, "Invalid Key"))?,
        query.interval_seconds,
    );

    {
        let mut state = key_state.write().expect("write to state");

        let new_state = KeyState::Repeating(key, interval_seconds);

        if *state == new_state {
            return Ok("Already Repeating Key");
        } else {
            *state = new_state;
        }
    }

    let state = Arc::clone(key_state.inner());
    let mut enigo = Enigo::new();

    thread::spawn(move || {
        let duration: Duration = Duration::from_secs(interval_seconds);

        while *state.read().expect("read state") == KeyState::Repeating(key, interval_seconds) {
            enigo.key_click(key);
            thread::sleep(duration);
        }
    });

    Ok("Started Repeating Key")
}

#[get("/repeat/stop")]
pub fn stop_repeating(key_state: &State<Arc<RwLock<KeyState>>>) -> &'static str {
    let mut state = key_state.write().expect("write to state");
    *state = KeyState::Idle;

    "Stopped Repeating"
}

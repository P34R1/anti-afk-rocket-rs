use enigo::{Enigo, KeyboardControllable};
use rocket::State;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::api::key_types::KeyState;

#[derive(FromForm)]
pub struct Repeat<'r> {
    #[field(validate = len(1..=1))]
    letter: &'r str,
    interval_seconds: u64,
}

#[get("/repeat/start?<query..>")]
pub fn start_repeating(key_state: &State<Arc<RwLock<KeyState>>>, query: Repeat) -> &'static str {
    let Repeat {
        letter,
        interval_seconds,
    } = query;
    let key = enigo::Key::Layout(letter.chars().next().expect("get letter"));

    if *key_state.read().expect("read state") == KeyState::Repeating(key, query.interval_seconds) {
        return "Already Started Repeating Same Keys";
    }
    *key_state.write().expect("write to state") = KeyState::Repeating(key, query.interval_seconds);

    let state = Arc::clone(key_state.inner());

    thread::spawn(move || {
        let mut enigo = Enigo::new();
        loop {
            let (repeat_key, interval) = match *state.read().unwrap() {
                // If anything changes
                KeyState::Repeating(repeat_key, interval)
                    if repeat_key != key || interval != interval_seconds =>
                {
                    break
                }

                KeyState::Repeating(key, interval) => (key, interval),
                KeyState::Idle => break,
            };

            enigo.key_click(repeat_key);
            thread::sleep(Duration::from_secs(interval));
        }
    });

    "Successfully Started Repeating"
}

#[get("/repeat/stop")]
pub fn stop_repeating(key_state: &State<Arc<RwLock<KeyState>>>) -> &'static str {
    let mut state = key_state.write().expect("write to state");
    *state = KeyState::Idle;

    "Successfully Stopped Repeating"
}

use enigo::{Enigo, KeyboardControllable};
use rocket::State;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use super::KeyState;

#[derive(FromForm)]
pub struct Repeat<'r> {
    #[field(validate = len(1..=1))]
    letter: &'r str,
    interval_seconds: u64,
}

fn repeat_key(state: Arc<RwLock<KeyState>>, interval_seconds: u64, key: enigo::Key) {
    let mut enigo = Enigo::new();
    while *state.read().expect("read state") == KeyState::Repeating(key, interval_seconds) {
        enigo.key_click(key);

        thread::sleep(Duration::from_secs(interval_seconds));
    }
}

fn handle_state(
    key_state: &RwLock<KeyState>,
    key: enigo::Key,
    interval_seconds: u64,
) -> Result<(), ()> {
    if *key_state.read().expect("read state") == KeyState::Repeating(key, interval_seconds) {
        return Err(());
    }

    *key_state.write().expect("write to state") = KeyState::Repeating(key, interval_seconds);
    Ok(())
}

#[get("/repeat/start?<query..>")]
pub fn start_repeating(key_state: &State<Arc<RwLock<KeyState>>>, query: Repeat) -> String {
    let Repeat {
        letter,
        interval_seconds,
    } = query;

    let key = enigo::Key::Layout(letter.chars().next().expect("get letter"));

    if handle_state(key_state, key, interval_seconds).is_err() {
        return format!("Already Repeating {}", letter);
    }

    let state = Arc::clone(key_state.inner());

    thread::spawn(move || repeat_key(state, interval_seconds, key));

    String::from("Successfully Started Repeating")
}

#[get("/repeat/stop")]
pub fn stop_repeating(key_state: &State<Arc<RwLock<KeyState>>>) -> &'static str {
    let mut state = key_state.write().expect("write to state");
    *state = KeyState::Idle;

    "Successfully Stopped Repeating"
}

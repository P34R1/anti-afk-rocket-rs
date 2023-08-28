use enigo::{Enigo, KeyboardControllable};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

#[derive(Debug, PartialEq)]
pub enum KeyState {
    Idle,
    Repeating(enigo::Key, u64),
}

pub struct KeyHandler {
    state: Arc<RwLock<KeyState>>,
}

impl KeyHandler {
    pub fn new() -> Self {
        KeyHandler {
            state: Arc::new(RwLock::new(KeyState::Idle)),
        }
    }

    pub fn start_repeating(&self, key: enigo::Key, interval_seconds: u64) {
        let state = self.state.clone();

        if *state.read().expect("read state") == KeyState::Repeating(key, interval_seconds) {
            return;
        }
        *state.write().expect("write to state") = KeyState::Repeating(key, interval_seconds);

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
    }

    pub fn stop_repeating(&self) {
        let mut state = self.state.write().expect("write to state");
        *state = KeyState::Idle;
    }
}

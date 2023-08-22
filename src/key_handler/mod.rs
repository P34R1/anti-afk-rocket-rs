use enigo::{Enigo, KeyboardControllable};
use std::{thread, time::Duration};

use crate::api::presskey::{FailedKeyPress, KeyPress};

pub struct KeyHandler;

impl KeyHandler {
    pub fn press_keys(&self, text: String) -> Result<KeyPress, FailedKeyPress> {
        if text.is_empty() {
            let err_text = String::from("no key given");
            Err(FailedKeyPress { err_text })
        } else {
            let mut enigo = Enigo::new();

            thread::sleep(Duration::from_secs(1));
            enigo.key_sequence(&text);

            Ok(KeyPress { pressed_keys: text })
        }
    }
}

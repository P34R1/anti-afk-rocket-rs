use enigo::{Enigo, Key, KeyboardControllable};
// use std::{thread, time::Duration};

fn main() {
    let mut enigo = Enigo::new();

    enigo.key_click(Key::Layout('='));
}

use termion::input::TermRead;
use termion::event::{Event, Key};
use std::collections::HashMap;

pub struct Controller {
    pressed_key: Option<Key>
}

impl Controller {
    pub fn new() -> Self {
        Controller{
            pressed_key: None
        }
    }

    /// Update input info (from polling stdin)
    pub fn update(&mut self) {
        self.pressed_key = None;

        let stdin = std::io::stdin();

        for raw_key in stdin.keys() {
            let key = raw_key.expect("Failed to get key");

            self.pressed_key = Some(key);
            break;
        }
    }

    pub fn get_pressed_key(&self) -> Option<Key> {
        self.pressed_key
    }
}
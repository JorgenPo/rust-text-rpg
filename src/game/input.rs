use termion::input::{TermRead, Keys};
use termion::event::{Event, Key};
use std::collections::HashMap;
use termion::AsyncReader;
use std::rc::Rc;

pub struct Controller {
    reader: Keys<AsyncReader>
}

impl Controller {
    pub fn new() -> Self {
        Controller{
            reader: termion::async_stdin().keys(),
        }
    }

    pub fn get_pressed_key(&mut self) -> Option<Key> {
        return match self.reader.next() {
            Some(key) => Some(key.unwrap()),
            None => None,
        };
    }
}
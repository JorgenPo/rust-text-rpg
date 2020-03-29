use crate::render::{Render, Coordinate};
use log::{Metadata, Record};
use crate::widgets::label::Label;
use std::cell::RefCell;

pub trait Logger {
    fn info(&mut self, message: &str);
    fn warn(&mut self, message: &str);
}
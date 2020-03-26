use crate::render::{Drawable, Render, Position};

use termion::color::Color;
use termion::color;
use std::cmp::max;

pub struct Label {
    pub color: Box<dyn Color>,
    pub selected_color: Box<dyn Color>,
    pub text: String,
    pub position: Position,
    pub selected: bool
}

impl Label {
    pub fn new(text: &str) -> Self {
        let default = Label::default();
        Label { text: String::from(text), .. default}
    }

    pub fn set_color(&mut self, color: Box<dyn Color>) -> &mut Self {
        self.color = color;
        self
    }

    pub fn set_selected_color(&mut self, color: Box<dyn Color>) -> &mut Self {
        self.selected_color = color;
        self
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }
}

impl Drawable for Label {
    fn draw(&self) -> String {
        let color = match self.selected {
            true => self.selected_color.as_ref(),
            false => self.color.as_ref()
        };

        format!("{}{}", color::Fg(color), self.text)
    }

    fn get_width(&self) -> u16 {
        max(self.text.len() as u16, 1)
    }

    fn get_height(&self) -> u16 {
        1
    }

    fn get_position(&self) -> &Position {
       &self.position
    }

    fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }
}

impl Default for Label {
    fn default() -> Self {
        Label {
            color: Box::new(color::Black),
            selected_color: Box::new(color::Green),
            text: "".to_string(),
            position: Position::from(1, 1),
            selected: false
        }
    }
}
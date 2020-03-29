//! Exit splash screen state

use crate::game::{GlobalState, PlayResult};
use std::time::Duration;
use crate::widgets::label::Label;
use crate::render::{Drawable, Position, Coordinate};
use std::io::Write;

pub struct PlayState {

}

impl super::PlayState for PlayState {
    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult {
        let mut brightness: u8 = 255;
        let mut inverted: u8 = 0;

        let mut rust_label = Label::new("R U S T");
        let mut rpg_label = Label::new("R P G");
        let mut author_label = Label::new("Made by George Popoff using Rust:3");

        rust_label.set_position(Position {
            x: Coordinate::Centered,
            y: Coordinate::Absolute(game_state.render.term_size.height / 2),
        });

        rpg_label.set_position(Position {
            x: Coordinate::Centered,
            y: Coordinate::Absolute(game_state.render.term_size.height / 2 + 1),
        });

        author_label.set_position(Position {
            x: Coordinate::Centered,
            y: Coordinate::Absolute(game_state.render.term_size.height - 3)
        });

        for _i in 0..255 {
            game_state.render.clear_color =
                Box::new(termion::color::Rgb(brightness, brightness, brightness));
            game_state.render.clear_screen();

            rust_label.color = Box::new(termion::color::Rgb(inverted, inverted, inverted));
            rpg_label.color = Box::new(termion::color::Rgb(inverted, inverted, inverted));
            author_label.color = Box::new(termion::color::Rgb(inverted, inverted, inverted));

            game_state.render.draw(&rust_label);
            game_state.render.draw(&rpg_label);
            game_state.render.draw(&author_label);

            brightness = brightness - 1;
            inverted = inverted + 1;

            std::thread::sleep(Duration::from_millis(20));
        }

        PlayResult::Pop
    }

    fn to_string(&self) -> String {
        String::from("ExitSplash")
    }
}

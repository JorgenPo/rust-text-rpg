//! Starting splash screen

use crate::game::{GlobalState, PlayResult};
use termion::color;
use std::time::Duration;
use crate::widgets::label::Label;
use crate::render::{Drawable, Position};
use crate::render::Coordinate::{Centered, Absolute};

pub struct PlayState {

}

fn lerp_color(start: color::Rgb, end: color::Rgb, k: f32) -> color::Rgb {

    let r = start.0 as f32 + ((end.0 as f32 - start.0 as f32) * k);
    let g = start.1 as f32 + ((end.1 as f32 - start.1 as f32) * k);
    let b = start.2 as f32 + ((end.2 as f32 - start.2 as f32) * k);

    color::Rgb(r as u8, g as u8, b as u8)
}

impl PlayState {
    pub fn new() -> Self {
        PlayState{}
    }
}

const SPLASH_DURATION: Duration = Duration::from_secs(5);

impl super::PlayState for PlayState {

    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult {
        game_state.render.set_cursor_position((1, 1));

        let finish_color = color::Rgb(10, 10, 10);
        let start_color = color::Rgb(50, 50, 50);

        let mut org_label = Label::new("Breeze software presents");
        let mut game_label = Label::new("T H E | G A M E");

        game_label.set_color(Box::new(color::Rgb(255, 255, 255)));
        org_label.set_color(Box::new(color::Rgb(255, 255, 255)));
        game_label.set_position(Position {
            x: Centered,
            y: Absolute(game_state.render.term_size.height / 2),
        });
        org_label.set_position(Position {
            x: Centered,
            y: Absolute(game_state.render.term_size.height / 2 - 2),
        });

        for x in 1..game_state.render.term_size.width {
            for y in 1..game_state.render.term_size.height {
                let k = y as f32 / game_state.render.term_size.width as f32;
                let color = lerp_color(start_color, finish_color, k);

                game_state.render.set_pixel_color((x, y), Box::new(color));
            }
        }

        game_state.render.draw(&game_label);
        game_state.render.draw(&org_label);

        game_state.render.flash();
        std::thread::sleep(SPLASH_DURATION);

        let to_white_diff = 255 - finish_color.0;
        let mut current_color = finish_color.0;
        for x in 1..to_white_diff {
            game_state.render.clear_color =
                Box::new(color::Rgb(current_color, current_color, current_color));
            current_color = current_color + 1;

            game_state.render.clear_screen();

            std::thread::sleep(Duration::from_millis(5));
        }

        PlayResult::Switch(Box::new(super::menu::PlayState::new(game_state)))
    }

    fn to_string(&self) -> String {
        String::from("StartSplash")
    }

    fn is_waiting_for_input(&self) -> bool {
        false
    }
}
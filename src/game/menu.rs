use crate::game::{GlobalState, PlayResult};
use crate::widgets::label::Label;
use crate::render::{Drawable, Position, Coordinate};

use termion::color;
use termion::event::Key;
use std::time::Duration;

use super::map;
use super::exit_splash;

/// Menu GAME state implementation

pub struct PlayState {
    buttons: [Label; 3],
    selected_index: i8,
    bg_music_started: bool
}

const BUTTON_NEW_GAME: u8 = 0;
const BUTTON_SETTINGS: u8 = 1;
const BUTTON_EXIT: u8 = 2;

const BUTTON_SOUND_EXIT: &'static str = "assets/sound/button2.wav";
const BUTTON_SOUND: &'static str = "assets/sound/button.wav";

impl PlayState {
    pub fn new(state: &GlobalState) -> Self {

        let new_game = Label::new("New GAME");
        let settings = Label::new("Settings");
        let exit = Label::new("Exit");

        let mut this = PlayState {
            buttons: [
                new_game,
                settings,
                exit
            ],
            selected_index: 0,
            bg_music_started: false
        };

        for (i, button) in this.buttons.iter_mut().enumerate() {
            let y = state.render.term_size.height as f32 * 0.2
                + (i * 5) as f32;

            button.set_position(Position{
                x: Coordinate::Centered,
                y: Coordinate::Absolute(y as u16)
            });
        }

        this.buttons[this.selected_index as usize].set_selected(true);

        this
    }

    fn on_button_pressed(&mut self, button: u8, game_state: &mut GlobalState) -> PlayResult {
        return match button {
            BUTTON_EXIT => {
                game_state.sound.play(BUTTON_SOUND_EXIT).unwrap();
                std::thread::sleep(Duration::from_millis(800));
                return PlayResult::Switch(Box::new(exit_splash::PlayState{}));
            },
            BUTTON_NEW_GAME => {
                return PlayResult::Push(Box::new(map::PlayState::new()))
            }
            _ => PlayResult::Still
        };
    }

    fn adjust_selected_index(&mut self) {
        self.selected_index = self.selected_index % self.buttons.len() as i8;

         if self.selected_index < 0 {
             self.selected_index = (self.buttons.len() as i8) + self.selected_index;
         }
    }

    fn render(&mut self, game_state: &mut GlobalState) {

        for button in &self.buttons {
            game_state.render.draw(button);
        }
    }
}

const BG_MUSIC: &'static str = "assets/sound/menu_bg.wav";

impl super::PlayState for PlayState {

    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult {
        if !self.bg_music_started {
            game_state.sound.play(BG_MUSIC).unwrap();
            self.bg_music_started = true;
        }

        game_state.render.clear_color = Box::new(color::Rgb(255, 255, 255));
        game_state.render.clear_screen();

        self.render(game_state);

        PlayResult::Still
    }

    fn to_string(&self) -> String {
        String::from("MenuState")
    }

    fn on_key_pressed(&mut self, game_state: &mut GlobalState, key: Key) -> PlayResult {
        self.buttons[self.selected_index as usize].set_selected(false);

        match key {
            Key::Down => {
                self.selected_index = self.selected_index + 1;
                game_state.sound.play(BUTTON_SOUND).unwrap();
            }
            Key::Up => {
                self.selected_index = self.selected_index - 1;
                game_state.sound.play(BUTTON_SOUND).unwrap();
            }
            Key::Esc => {
                return PlayResult::Push(Box::new(map::PlayState::new()));
            }
            Key::Char(char) => {
                // Enter on exit label
                if (char as u8) == 10 {
                    return self.on_button_pressed(self.selected_index as u8, game_state);
                }
            }
            _ => {}
        }

        self.adjust_selected_index();

        self.buttons[self.selected_index as usize].set_selected(true);
        PlayResult::Still
    }
}
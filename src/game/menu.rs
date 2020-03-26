use crate::game::{GlobalState, PlayResult};
use crate::widgets::label::Label;
use crate::render::{Drawable, Position, Coordinate};

use termion::color;
use termion::event::Key;
use std::time::Duration;

/// Menu game state implementation

pub struct PlayState {
    buttons: [Label; 3],
    selected_index: i8,
    should_terminate: bool,
    bg_music_started: bool
}

impl PlayState {
    pub fn new(state: &GlobalState) -> Self {

        let new_game = Label::new("New game");
        let settings = Label::new("Settings");
        let exit = Label::new("Exit");

        let mut this = PlayState {
            buttons: [
                new_game,
                settings,
                exit
            ],
            selected_index: 0,
            should_terminate: false,
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

        this
    }

    fn update(&mut self, game_state: &mut GlobalState) {
        const BUTTON_EXIT_SOUND: &'static str = "assets/sound/button2.wav";
        const BUTTON_SOUND: &'static str = "assets/sound/button.wav";

        self.should_terminate = false;

        self.buttons[self.selected_index as usize].set_selected(false);

        match game_state.input.get_pressed_key() {
            Some(Key::Down) => {
                self.selected_index = self.selected_index + 1;
                game_state.sound.play(BUTTON_SOUND);
            }
            Some(Key::Up) => {
                self.selected_index = self.selected_index - 1;
                game_state.sound.play(BUTTON_SOUND);
            }
            Some(Key::Esc) => {
                self.should_terminate = true;
            }
            Some(Key::Char(char)) => {
                // Enter on exit label
                if (char as u8) == 10 && self.selected_index == 2 {
                    self.should_terminate = true;
                    game_state.sound.play(BUTTON_EXIT_SOUND);
                    std::thread::sleep(Duration::from_millis(800));
                }
            }
            _ => {}
        }

        self.adjust_selected_index();

        self.buttons[self.selected_index as usize].set_selected(true);
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
            game_state.sound.play(BG_MUSIC);
            self.bg_music_started = true;
        }

        game_state.render.clear_color = Box::new(color::Rgb(255, 255, 255));
        game_state.render.clear_screen();

        self.update(game_state);
        self.render(game_state);

        if self.should_terminate {
            return PlayResult::Switch(Box::new(super::exit_splash::PlayState{}));
        }

        PlayResult::Still
    }

    fn to_string(&self) -> String {
        String::from("MenuState")
    }

    fn is_waiting_for_input(&self) -> bool {
        true
    }
}
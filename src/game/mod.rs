use std::error::Error;
use std::fmt::Debug;

use crate::render;

use std::time::Duration;
use std::io::Write;
use termion::raw::IntoRawMode;
use termion::screen;
use log::{info, error};

use crate::widgets::label::Label;
use crate::render::Coordinate;
use std::panic::PanicInfo;
use std::any::TypeId;
use backtrace::Backtrace;
use termion::event::Key;

mod menu;
mod exit_splash;
mod start_splash;
mod input;
mod sound;
mod map;
mod loggers;

/// Here is a state system implemented
/// Inspired by Veloren project

enum PlayResult {
    /// Terminate the GAME
    Shutdown,
    /// Pop the last screen. If the last screen is the only screen, then it is equal to
    /// Shutdown.
    Pop,
    /// Push a new state to the stack
    Push(Box<dyn PlayState>),
    /// Replace the current state with another
    Switch(Box<dyn PlayState>),
    /// Do nothing
    Still,
}

/// Represents some GAME state (e.g. menu, battle and so on)
trait PlayState {
    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult;
    fn to_string(&self) -> String;
    fn on_key_pressed(&mut self, game_state: &mut GlobalState, key: Key) -> PlayResult {
        PlayResult::Still
    }
}

pub struct GlobalState {
    render: render::Render,
    input: input::Controller,
    sound: sound::Manager,
}

impl GlobalState {
    pub fn new() -> GlobalState {

        GlobalState {
            render: render::Render::new(),
            input: input::Controller::new(),
            sound: sound::Manager::new(),
        }
    }


}

pub struct Game {
    states: Vec<Box<dyn PlayState>>,
    fps: u64,
    mpf: u64
}

impl Game {
    pub fn new() -> Self {
        let states : Vec<Box<dyn PlayState>> = vec![];
        let fps = 10;
        let mpf = 1000 / fps;

        Game { states, fps, mpf }
    }

    /// Starts the GAME
    pub fn run(&mut self) -> Result<(), String> {
        let mut stdout = std::io::stdout().into_raw_mode();

        if stdout.is_err() {
            return Err(format!("Failed to set terminal into raw mode"));
        }

        let mut stdout = screen::AlternateScreen::from(stdout.unwrap());
        let mut global_state = GlobalState::new();

        self.states.push(Box::new(start_splash::PlayState::new()));

        std::panic::set_hook(Box::new(panic_handler));

        let mut playing = true;
        while playing {
            let mut current_state = match self.states.last_mut() {
                None => {
                    playing = false;
                    continue;
                },
                Some(state) => state
            };

            info!("Current state: {}", current_state.to_string());

            let mut result = PlayResult::Still;
            if let Some(key) = global_state.input.get_pressed_key() {
                result = current_state.on_key_pressed(&mut global_state, key);

                match key {
                    Key::Esc => {
                        info!("Esc pressed. Exit game from state {}", current_state.to_string());
                        break;
                    },
                    _ => {}
                }
            }

            if let PlayResult::Still = result {
                result = current_state.play(&mut global_state);
            }

            match result {
                PlayResult::Shutdown => {
                    info!("Shutdown state");
                    playing = false;
                },
                PlayResult::Pop => {
                    info!("Pop state");
                    self.states.pop().expect("Empty state in queue!");
                },
                PlayResult::Push(state) => {
                    info!("Push state: {}", state.to_string());
                    self.states.push(state);
                },
                PlayResult::Switch(state) => {
                    info!("Switch to state: from {} to {}", current_state.to_string(), state.to_string());
                    self.states.pop().expect("Empty state in queue!");
                    self.states.push(state);
                    continue;
                },
                PlayResult::Still => {}
            }

            stdout.flush().unwrap();
            std::thread::sleep(Duration::from_millis(self.mpf));
        }

        info!("Shutdown the GAME");
        Ok(())
    }
}

fn panic_handler(info: &PanicInfo) {
    error!("Panic payload is {:?}", info.payload().type_id());

    let backtrace = Backtrace::new();
    if let Some(s) = info.payload().downcast_ref::<String>() {
        if let Some(location) = info.location() {
            error!("Panic: {} in {}", s, location);
        } else {
            error!("Panic: {}", s);
        }
    } else {
        if let Some(location) = info.location() {
            error!("Some critical error occurred in {}:{}!", location.file(), location.line());
        } else {
            error!("Some critical error occurred!");
        }
    }

    error!("Backtrace: {:?}", backtrace);
}
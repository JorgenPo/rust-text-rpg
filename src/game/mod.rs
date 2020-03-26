use std::error::Error;
use std::fmt::Debug;

use crate::render;

use std::time::Duration;
use std::io::Write;
use termion::raw::IntoRawMode;
use termion::screen;

mod menu;
mod exit_splash;
mod start_splash;
mod input;
mod sound;

/// Here is a state system implemented
/// Inspired by Veloren project

enum PlayResult {
    /// Terminate the game
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

/// Represents some game state (e.g. menu, battle and so on)
trait PlayState {
    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult;
    fn to_string(&self) -> String;
    fn is_waiting_for_input(&self) -> bool;
}

pub struct GlobalState {
    render: render::Render,
    input: input::Controller,
    sound: sound::Manager,
}

pub struct Game {
    states: Vec<Box<dyn PlayState>>,
    fps: u16,
    mpf: u16
}

impl Game {
    pub fn new() -> Self {
        let states : Vec<Box<dyn PlayState>> = vec![];
        let fps = 10;
        let mpf = 1000 / fps;

        Game { states, fps, mpf }
    }

    /// Starts the game
    pub fn run(&mut self) -> Result<(), String> {
        let mut stdout = std::io::stdout().into_raw_mode();

        if stdout.is_err() {
            return Err(format!("Failed to set terminal into raw mode"));
        }

        let mut stdout = screen::AlternateScreen::from(stdout.unwrap());

        let mut global_state = GlobalState{
            render: render::Render::new(),
            input: input::Controller::new(),
            sound: sound::Manager::new(),
        };

        self.states.push(Box::new(start_splash::PlayState::new()));

        let mut playing = true;
        while playing {
            let mut current_state = match self.states.last_mut() {
                None => {
                    playing = false;
                    continue;
                },
                Some(state) => state
            };

            let waiting_for_input = current_state.is_waiting_for_input();
            match current_state.play(&mut global_state) {
                PlayResult::Shutdown => {
                    playing = false;
                },
                PlayResult::Pop => {
                    let state = self.states.pop().expect("Empty state in queue!");
                    println!("Pop a state: {}", state.to_string());
                },
                PlayResult::Push(state) => {
                    println!("Push new state: {}", state.to_string());
                    self.states.push(state);
                },
                PlayResult::Switch(state) => {
                    self.states.pop().expect("Empty state in queue!");
                    self.states.push(state);
                    continue;
                },
                PlayResult::Still => {}
            }

            stdout.flush().unwrap();

            if playing && waiting_for_input {
                // Update inputs
                global_state.input.update();

                // Wait some time to fixate FPS
                // std::thread::sleep(Duration::from_millis(self.mpf as u64));
            }
        }

        println!("Shutdown the game");
        Ok(())
    }
}
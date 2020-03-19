use std::error::Error;
use std::fmt::Debug;

mod menu;

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
    Switch(Box<dyn PlayState>)
}

/// Represents some game state (e.g. menu, battle and so on)
trait PlayState {
    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult;
    fn to_string(&self) -> String;
}

pub struct GlobalState {
}

pub struct Game {
    states: Vec<Box<dyn PlayState>>
}

impl Game {
    pub fn new() -> Self {
        let states : Vec<Box<dyn PlayState>> = vec![Box::new(menu::PlayState{})];
        Game { states }
    }

    /// Starts the game
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut global_state = GlobalState{};

        while let Some(result) = self.states.last_mut()
            .map(|state| state.play(&mut global_state)) {

            match result {
                PlayResult::Shutdown => {
                    println!("Shutdown");
                    return Ok(());
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
                    let old_state = self.states.pop().expect("Empty state in queue!");
                    println!("Switch to state: {} from {}", state.to_string(), old_state.to_string());
                    self.states.push(state);
                },
            }
        }

        Ok(())
    }
}
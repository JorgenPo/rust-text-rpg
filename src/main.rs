use rust_rpg::*;
use std::process;

fn main() {
    let mut game_state = game::Game::new();
    if let Err(error) = game_state.run() {
        println!("Game crashed: {}", error);
        process::exit(1);
    }

    println!("Thanks for playing! Goodbye!");
}

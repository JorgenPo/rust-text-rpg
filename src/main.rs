use rust_rpg::*;
use std::process;

fn main() {
    println!("The Rust RPG");
    println!("Written by George Popoff");
    println!("2020 (c)");

    let mut game_state = game::Game::new();
    if let Err(error) = game_state.run() {
        println!("Game crashed: {}", error);
        process::exit(1);
    }

    println!("Thanks for playing! Goodbye!");
}

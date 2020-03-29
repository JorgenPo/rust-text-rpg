use rust_rpg::*;
use std::process;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Config, Appender, Root};
use log::LevelFilter;

fn main() {
    // Init logger
    let appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l}: {m}\n")))
        .append(false)
        .build("game.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("filelog", Box::new(appender)))
        .build(Root::builder().appender("filelog").build(LevelFilter::Trace))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    let mut game_state = game::Game::new();
    if let Err(error) = game_state.run() {
        println!("Game crashed: {}", error);
        process::exit(1);
    }

    println!("Thanks for playing! Goodbye!");
}

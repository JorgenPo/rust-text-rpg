use crate::game::{GlobalState, PlayResult};

/// Menu game state implementation

#[derive(Debug)]
pub struct PlayState {

}

impl super::PlayState for PlayState {
    fn play(&mut self, game_state: &mut GlobalState) -> PlayResult {
        PlayResult::Shutdown
    }

    fn to_string(&self) -> String {
        String::from("MenuState")
    }
}
use ashscript_types::intents::Intents;

use crate::game_state::GameState;

use super::shared::{AiFeatures, Memory};

pub fn main(game_state: &GameState, memory: &mut Memory) -> Intents {
    Intents::default()
}

pub fn spawn_units(game_state: &GameState, memory: &mut Memory) {
    // loop through factories
    // spawn based on need of new type of unit
}
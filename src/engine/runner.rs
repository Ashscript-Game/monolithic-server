use crate::game_state::GameState;

use super::{intents::get_and_process_intents, unit::{age_units, delete_dead_units}};

pub fn runner(game_state: &mut GameState) {
    get_and_process_intents(game_state);

    age_units(game_state);
    delete_dead_units(game_state);


}
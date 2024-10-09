use crate::game_state::GameState;

use super::unit::{age_units, delete_dead_units};

pub fn runner(game_state: &mut GameState) {
    age_units(game_state);
    delete_dead_units(game_state);


}
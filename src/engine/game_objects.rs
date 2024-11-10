use ashscript_types::{components::{energy::Energy, health::{self, Health}, storage::Storage}};
use enum_iterator::all;

use crate::game_state::GameState;

// The following update functinos are likely only temporary measures while action processing is part of the engine

pub fn update_resources(game_state: &mut GameState) {
    for (entity, storage) in game_state.world.query_mut::<&mut Storage>() {
        storage.future_resources = storage.resources.clone();
    }
}

// pub fn update_health(game_state: &mut GameState) {
//     for (entity, health) in game_state.world.query_mut::<&mut Health>() {
//         health.future = health.current;
//     }
// }

// pub fn update_energy(game_state: &mut GameState) {
//     for (entity, energy) in game_state.world.query_mut::<&mut Energy>() {
//         energy.future = energy.current;
//     }
// }

use ashscript_types::intents::Intents;

use crate::game_state::{self, GameState};

use super::shared::{AiFeatures, Memory, UnitRole};

pub fn main(game_state: &GameState, memory: &mut Memory) -> Intents {
    let mut intents = Intents::default();

    organize_units(game_state, memory);

    scouts_scout(game_state, memory);
    attackers_attack(game_state, memory);
    defenders_defend(game_state, memory);
    extractors_extract(game_state, memory);
    haulers_haul(game_state, memory);
    turrets_shoot(game_state, memory);


    spawn_units(game_state, memory);

}

pub fn spawn_units(game_state: &GameState, memory: &mut Memory) {
    // loop through factories
    // spawn based on need of new type of unit
}

pub fn organize_units(game_state: &GameState, memory: &mut Memory) {
    for chunk in game_state.map.chunks.values() {
        for unit in chunk.units.values() {

        }
    }
}

pub fn scouts_scout(game_state: &GameState, memory: &mut Memory) {
    for unit_id in memory.units_by_role[UnitRole::Scout].iter() {
        // get the unit by its id
        // run scout logic
    }
}

pub fn attackers_attack(game_state: &GameState, memory: &mut Memory) {
    for unit_id in memory.units_by_role[UnitRole::Attacker].iter() {
        // get the unit by its id
        // run attack logic
    }
}

pub fn defenders_defend(game_state: &GameState, memory: &mut Memory) {
    for unit_id in memory.units_by_role[UnitRole::Defender].iter() {
        // get the unit by its id
        // run defend logic
    }
}

pub fn extractors_extract(game_state: &GameState, memory: &mut Memory) {
    for unit_id in memory.units_by_role[UnitRole::Extractor].iter() {
        // get the unit by its id
        // run extract logic
    }
}

pub fn haulers_haul(game_state: &GameState, memory: &mut Memory) {
    for unit_id in memory.units_by_role[UnitRole::Hauler].iter() {
        // get the unit by its id
        // run haul logic
    }
}

pub fn turrets_shoot(game_state: &GameState, memory: &mut Memory) {
    for chunk in game_state.map.chunks.values() {
        // loop through turrets
        // shoot at closest enemy
    }
}
use std::u32;

use ashscript_types::{
    actions::UnitAttack,
    player::PlayerId,
    resource::Resource,
    unit::{Unit, UnitBody},
};
use hexx::Hex;

use crate::game_state::GameState;

pub fn age_units(game_state: &mut GameState) {
    for (_, chunk) in game_state.map.chunks.iter_mut() {
        for (_, unit) in chunk.units.iter_mut() {
            unit.age += 1;

            // Age also increases based on how much uranium is being carried
            unit.age -= unit.storage.resources.get(&Resource::Uranium).unwrap_or(&0) / 100;
        }
    }
}

pub fn units_generate_energy(game_state: &mut GameState) {
    for (_, chunk) in game_state.map.chunks.iter_mut() {
        for (_, unit) in chunk.units.iter_mut() {
            unit.energy =
                (unit.energy + unit.body.energy_income()).max(unit.body.energy_capacity());
        }
    }
}

pub fn delete_dead_units(game_state: &mut GameState) {
    for (_, chunk) in game_state.map.chunks.iter_mut() {
        chunk.units.retain(|_, unit| {
            if unit.age >= unit.max_age() {
                return false;
            }
            if unit.health == 0 {
                return false;
            }

            true
        });
    }
}

pub fn attack_intents(game_state: &mut GameState, attack_intents: &Vec<UnitAttack>) {
    for intent in attack_intents {}
}

pub fn can_attack(game_state: &GameState, intent: &UnitAttack) -> bool {
    true
}

pub fn attack(attacker: &mut Unit, target: &mut Unit) {
    let cost = attacker.attack_cost();
    if attacker.energy < cost {
        return;
    }

    if attacker.hex == target.hex {
        return;
    }

    let distance = attacker.hex.unsigned_distance_to(target.hex);
    if distance > attacker.range() {
        return;
    }

    let damage = attacker.damage();
    if damage > target.health {
        target.health = 0
    } else {
        target.health -= damage
    }

    attacker.energy -= cost;
}

pub fn spawn_unit(
    hex: Hex,
    name: String,
    body: UnitBody,
    owner_id: PlayerId,
    game_state: &mut GameState,
) {
    let Some(chunk) = game_state.map.chunk_at_mut(&hex) else {
        return;
    };

    chunk
        .units
        .insert(hex, Unit::new(hex, name, body, owner_id));
}

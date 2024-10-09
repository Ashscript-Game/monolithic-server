use std::u32;

use ashscript_types::unit::{Unit, UnitPart};

use crate::game_state::GameState;

use super::constants::{AGE_PER_GEN_PART, UNIT_AGE_EXP, UNIT_BASE_AGE, UNIT_PART_WEIGHTS};

pub fn age_units(game_state: &mut GameState) {
    for (_, unit) in game_state.units.iter_mut() {
        age(unit);
    }
}

pub fn age(unit: &mut Unit) {
    unit.age += 1;
}

pub fn delete_dead_units(game_state: &mut GameState) {
    game_state.units.retain(|_, unit| {
        if unit.age >= unit_max_age(unit) {
            return false
        }
        if unit.health == 0 {
            return false
        }

        true
    });
}

pub fn attack(attacker: &mut Unit, target: &mut Unit) {
    if attacker.energy < unit_attack_cost(attacker) {
        return;
    }

    if attacker.hex == target.hex {
        return;
    }

    let distance = attacker.hex.unsigned_distance_to(target.hex);
    if distance > unit_range(attacker) {
        return;
    }

    let damage = unit_damage(attacker);
    if damage > target.health {
        target.health = 0
    } else {
        target.health -= damage
    }

    attacker.energy -= unit_attack_cost(attacker);
}

pub fn unit_range(unit: &Unit) -> u32 {
    unit.body[UnitPart::Ranged]
}

pub fn unit_damage(unit: &Unit) -> u32 {
    unit.body[UnitPart::Ranged]
}

pub fn unit_attack_cost(unit: &Unit) -> u32 {
    unit.body[UnitPart::Ranged]
}

pub fn unit_weight(unit: &Unit) -> u32 {
    let mut weight = 0;

    for (part, _) in UNIT_PART_WEIGHTS.iter() {
        weight += UNIT_PART_WEIGHTS[part]
    }
    
    weight
}

pub fn unit_max_age(unit: &Unit) -> u32 {
    ((unit.body[UnitPart::Generate] * AGE_PER_GEN_PART) as f32).powf(UNIT_AGE_EXP) as u32 + UNIT_BASE_AGE
}
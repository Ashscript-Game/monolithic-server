use std::u32;

use ashscript_types::unit::{Unit, UnitPart};

use super::constants::UNIT_PART_WEIGHTS;

pub fn age(unit: &mut Unit) {
    unit.age += 1;
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
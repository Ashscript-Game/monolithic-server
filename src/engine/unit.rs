use std::u32;

use ashscript_types::unit::{Unit, UnitPart};

pub fn age(unit: &mut Unit) {
    unit.age += 1;
}

pub fn attack(attacker: &mut Unit, target: &mut Unit) {

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
    u32::MAX
}
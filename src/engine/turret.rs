use ashscript_types::{structures::turret::Turret, unit::Unit};

pub fn turret_attack_cost(turret: &Turret) -> u32 {
    turret_range(turret) + turret_damage(turret)
}

pub fn turret_range(turret: &Turret) -> u32 {
    1
}

pub fn turret_damage(turret: &Turret) -> u32 {
    1
}

pub fn turret_attack(turret: &Turret, unit: &mut Unit) {
    if turret.energy < turret_attack_cost(turret) {
        return
    };

    if turret.hex == unit.hex {
        return
    }

    let distance = turret.hex.unsigned_distance_to(unit.hex);
    if distance > turret_range(&turret) {
        return
    }

    if turret_damage(&turret) > unit.health {
        unit.health = 0
    } else {
        unit.health -= turret_damage(&turret)
    }

    turret.energy -= turret_attack_cost(turret);
}
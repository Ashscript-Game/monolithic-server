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
    let attack_cost = turret.attack_cost();
    if turret.energy < attack_cost {
        return
    };
    
    if turret.hex == unit.hex {
        return
    }

    let distance = turret.hex.unsigned_distance_to(unit.hex);
    if distance > turret.range() {
        return
    }

    let damage = turret.damage();
    if damage > unit.health {
        unit.health = 0
    } else {
        unit.health -= damage
    }

    turret.energy -= attack_cost;
}
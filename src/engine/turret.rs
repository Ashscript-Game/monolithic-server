use ashscript_types::{components::{energy::Energy, health::Health, tile::Tile}, structures::turret::Turret, unit::Unit};
use hexx::Hex;

pub fn turret_attack_cost(turret: &Turret) -> u32 {
    turret_range(turret) + turret_damage(turret)
}

pub fn turret_range(turret: &Turret) -> u32 {
    1
}

pub fn turret_damage(turret: &Turret) -> u32 {
    1
}

pub fn turret_attack(turret: &Turret, turret_tile: Tile, turret_energy: &mut Energy, unit_tile: &Tile, unit_health: &mut Health) {
    let attack_cost = turret.attack_cost();
    if turret_energy.0 < attack_cost {
        return
    };
    
    if turret_tile.hex == unit_tile.hex {
        return
    }

    let distance = turret_tile.hex.unsigned_distance_to(unit_tile.hex);
    if distance > turret.range() {
        return
    }

    let damage = turret.damage();
    if damage > unit_health.0 {
        unit_health.0 = 0
    } else {
        unit_health.0 -= damage
    }

    turret_energy.0 -= attack_cost;
}
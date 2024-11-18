use ashscript_types::components::{energy::Energy, health::Health, tile::Tile, turret::Turret};
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
    if turret_energy.current < attack_cost {
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
    if damage > unit_health.current {
        unit_health.current = 0
    } else {
        unit_health.current -= damage
    }

    turret_energy.current = turret_energy.current.saturating_sub(attack_cost);
}
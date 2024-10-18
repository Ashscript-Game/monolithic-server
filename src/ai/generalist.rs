use ashscript_types::{
    intents::{FactorySpawnUnit, Intent, Intents, UnitAttack, UnitMove},
    objects::Attackable,
    terrain::Terrain,
    unit::{Unit, UnitBody, UnitPart},
};
use enum_map::enum_map;
use hexx::{shapes, Hex};

use crate::game_state::BotGameState;

use super::shared::{BotMemory, BotState, UnitRole};

pub fn main(game_state: &BotGameState, memory: &mut BotMemory) -> Intents {
    let mut intents = Intents::default();

    let mut bot_state = BotState::new();

    organize_units(game_state, memory, &mut bot_state);

    scouts_scout(game_state, memory);
    attackers_attack(game_state, memory, &mut bot_state, &mut intents);
    defenders_defend(game_state, memory);
    extractors_extract(game_state, memory);
    haulers_haul(game_state, memory);
    turrets_shoot(game_state, memory, &mut intents);
    factories_spawn_units(game_state, memory, &mut intents);

    spawn_units(game_state, memory);

    intents
}

pub fn spawn_units(game_state: &BotGameState, memory: &mut BotMemory) {
    // loop through factories
    // spawn based on need of new type of unit
}

pub fn organize_units(game_state: &BotGameState, memory: &mut BotMemory, bot_state: &mut BotState) {
    for chunk in game_state.map.chunks.values() {
        for unit in chunk.units.values() {
            if unit.owner_id != game_state.me.id {
                continue;
            };

            let role = match unit.name.as_str() {
                "attacker" => UnitRole::Attacker,
                "scout" => UnitRole::Scout,
                "defender" => UnitRole::Defender,
                "extractor" => UnitRole::Extractor,
                "hauler" => UnitRole::Hauler,
                _ => UnitRole::Unknown,
            };

            bot_state.unit_hexes_by_role[role].insert(unit.hex);
        }
    }
}

pub fn scouts_scout(game_state: &BotGameState, memory: &mut BotMemory) {
    for unit_id in memory.units_by_role[UnitRole::Scout].iter() {
        // get the unit by its id
        // run scout logic
    }
}

pub fn attackers_attack(
    game_state: &BotGameState,
    memory: &mut BotMemory,
    bot_state: &mut BotState,
    intents: &mut Intents,
) {
    for hex in bot_state.unit_hexes_by_role[UnitRole::Attacker].iter() {
        // get the unit by its id
        // run attack logic

        let Some(unit) = game_state.map.unit_at(hex) else {
            continue;
        };

        let nearby_enemy_hexes = find_enemy_hexes_in_range(game_state, *hex, unit.range());

        if let Some(enemy_hex) = nearby_enemy_hexes.first() {
            attack_enemy(game_state, unit, *enemy_hex, intents);
            move_unit(game_state, *hex, (*enemy_hex, unit.range()), intents);
            continue;
        };

        let Some(enemy_hex) = find_closest_enemy_hex(game_state, *hex) else {
            continue;
        };

        move_unit(game_state, *hex, (enemy_hex, unit.range()), intents);
    }
}

fn find_enemy_hexes(game_state: &BotGameState) -> Vec<Hex> {
    let mut enemy_hexes = Vec::new();

    for chunk in game_state.map.chunks.values() {
        for unit in chunk.units.values() {
            if unit.owner_id == game_state.me.id {
                continue;
            };

            enemy_hexes.push(unit.hex);
        }
    }

    enemy_hexes
}

fn find_closest_enemy_hex(game_state: &BotGameState, around: Hex) -> Option<Hex> {
    let mut closest_enemy_hex: Option<Hex> = None;
    let mut lowest_distance = u32::MAX;

    for chunk in game_state.map.chunks.values() {
        for enemy in chunk.units.values() {
            if enemy.owner_id == game_state.me.id {
                continue;
            };

            let distance = around.unsigned_distance_to(enemy.hex);
            if distance >= lowest_distance {
                continue;
            }

            closest_enemy_hex = Some(enemy.hex);
            lowest_distance = distance;
        }
    }

    closest_enemy_hex
}

fn find_enemy_hexes_in_range(game_state: &BotGameState, around: Hex, range: u32) -> Vec<Hex> {
    let mut enemy_hexes = find_enemy_hexes(game_state);

    for hex in shapes::hexagon(around, range) {
        let Some(unit) = game_state.map.unit_at(&hex) else {
            continue;
        };

        if unit.owner_id == game_state.me.id {
            continue;
        };

        enemy_hexes.push(unit.hex);
    }

    enemy_hexes
}

fn attack_enemy(game_state: &BotGameState, unit: &Unit, enemy_hex: Hex, intents: &mut Intents) {
    // decide wether to attack based on current energy, shield health, and move needs

    //

    intents.push(Intent::UnitAttack(UnitAttack {
        attacker_hex: unit.hex,
        target_hex: enemy_hex,
        target_kind: Attackable::Unit,
    }));
}

fn move_unit(
    game_state: &BotGameState,
    from_hex: Hex,
    (to_hex, to_range): (Hex, u32),
    intents: &mut Intents,
) {
    if from_hex.unsigned_distance_to(to_hex) <= to_range {
        return;
    }

    let unit_hexes = find_enemy_hexes(game_state);

    let path = hexx::algorithms::a_star(from_hex, to_hex, |_, bhex| {
        if bhex == to_hex || bhex == from_hex {
            return Some(1);
        }

        if let Some(terrain) = game_state.map.terrain_at(&bhex) {
            if *terrain == Terrain::Wall {
                return None;
            }
        }

        if unit_hexes.contains(&bhex) {
            return Some(5);
        }

        Some(1)
        /* (bhex != closest_enemy_hex &&/* bhex != closest_enemy_hex && ahex != unit_hex && */game_state.occupied_tiles.contains(&bhex)).then_some(1) */
    });

    if let Some(path) = path {
        if let Some(hex) = path.get(1) {
            intents.push(Intent::UnitMove(UnitMove {
                from: from_hex,
                to: *hex,
            }));
        }
    } else {
        println!("[basic combat ai] no path found");
    }
}

pub fn defenders_defend(game_state: &BotGameState, memory: &mut BotMemory) {
    for unit_id in memory.units_by_role[UnitRole::Defender].iter() {
        // get the unit by its id
        // run defend logic
    }
}

pub fn extractors_extract(game_state: &BotGameState, memory: &mut BotMemory) {
    for unit_id in memory.units_by_role[UnitRole::Extractor].iter() {
        // get the unit by its id
        // run extract logic
    }
}

pub fn haulers_haul(game_state: &BotGameState, memory: &mut BotMemory) {
    for unit_id in memory.units_by_role[UnitRole::Hauler].iter() {
        // get the unit by its id
        // run haul logic
    }
}

pub fn turrets_shoot(game_state: &BotGameState, memory: &mut BotMemory, intents: &mut Intents) {
    for chunk in game_state.map.chunks.values() {
        // loop through turrets
        // shoot at closest enemy

        for turret in chunk.turrets.values() {}
    }
}

pub fn factories_spawn_units(
    game_state: &BotGameState,
    memory: &mut BotMemory,
    intents: &mut Intents,
) {
    for chunk in game_state.map.chunks.values() {
        for factory in chunk.factories.values() {
            if factory.owner_id != game_state.me.id {
                continue;
            };

            intents.push(Intent::FactorySpawnUnit(FactorySpawnUnit {
                factory_hex: factory.hex,
                out: None,
                name: "attacker".to_string(),
                body: UnitBody(enum_map! {
                    UnitPart::Generate => 5,
                    UnitPart::Ranged => 1,
                    UnitPart::Shield => 1,
                    _ => 0,
                }),
            }));
        }
    }
}

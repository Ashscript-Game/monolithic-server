use ashscript_types::{
    components::{
        body::{UnitBody, UnitPart}, factory::Factory, owner::Owner, terrain::{self, Lava, Terrain, Wall}, tile::Tile, turret::Turret, unit::Unit
    },
    intents::{FactorySpawnUnit, Intent, Intents, UnitAttack, UnitMove},
    objects::GameObjectKind,
};
use hecs::{Entity, Or};
use hexx::{shapes, Hex};

use crate::game_state::BotGameState;

use super::shared::{BotMemory, BotState, UnitRole};

pub fn main(game_state: &mut BotGameState, memory: &mut BotMemory) -> Intents {
    let mut intents = Intents::default();

    let mut bot_state = BotState::new();

    println!("[generalist ai] tick: {}", game_state.global.tick);

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
    for (entity, (unit, tile, owner)) in game_state.world.query::<(&Unit, &Tile, &Owner)>().iter() {
        println!(
            "[generalist ai] found unit: {} at ({}, {})",
            unit.name, tile.hex.x, tile.hex.y
        );

        if owner.0 != game_state.me.id {
            continue;
        };

        let role = match unit.name.as_str() {
            "leader" => UnitRole::Leader,
            "attacker" => UnitRole::Attacker,
            "scout" => UnitRole::Scout,
            "defender" => UnitRole::Defender,
            "extractor" => UnitRole::Extractor,
            "hauler" => UnitRole::Hauler,
            _ => UnitRole::Unknown,
        };

        bot_state.unit_hexes_by_role[role].insert(tile.hex);
    }
}

pub fn scouts_scout(game_state: &BotGameState, memory: &mut BotMemory) {
    for unit_id in memory.units_by_role[UnitRole::Scout].iter() {
        // get the unit by its id
        // run scout logic
    }
}

pub fn attackers_attack(
    game_state: &mut BotGameState,
    memory: &mut BotMemory,
    bot_state: &mut BotState,
    intents: &mut Intents,
) {
    for hex in bot_state.unit_hexes_by_role[UnitRole::Attacker].iter() {
        // get the unit by its id
        // run attack logic

        let Some(unit_entity) = game_state.map.entity_at(hex, GameObjectKind::Unit) else {
            continue;
        };
        let mut unit_query = game_state
            .world
            .query_one::<(&Unit, &UnitBody, &Tile)>(*unit_entity)
            .unwrap();
        let Some((unit, unit_body, unit_tile)) = unit_query.get() else {
            continue;
        };

        println!(
            "[generalist ai] unit is trying to attack: {} at ({}, {})",
            unit.name, unit_tile.hex.x, unit_tile.hex.y
        );

        let nearby_enemy_hexes = find_enemy_hexes_in_range(game_state, *hex, unit_body.range());

        if let Some(enemy_hex) = nearby_enemy_hexes.first() {
            attack_enemy(game_state, unit_tile.hex, *enemy_hex, intents);
            move_unit(game_state, *hex, (*enemy_hex, unit_body.range()), intents);
            continue;
        };

        let Some(enemy_hex) = find_closest_enemy_hex(game_state, *hex) else {
            continue;
        };

        move_unit(game_state, *hex, (enemy_hex, unit_body.range()), intents);
    }
}

fn find_enemy_hexes(game_state: &BotGameState) -> Vec<Hex> {
    let mut enemy_hexes = Vec::new();

    for (entity, (unit, owner, tile)) in &mut game_state.world.query::<(&Unit, &Owner, &Tile)>() {
        if owner.0 != game_state.me.id {
            continue;
        };

        enemy_hexes.push(tile.hex);
    }

    enemy_hexes
}

fn find_closest_enemy_hex(game_state: &BotGameState, around: Hex) -> Option<Hex> {
    let mut closest_enemy_hex: Option<Hex> = None;
    let mut lowest_distance = u32::MAX;

    for (entity, (unit, owner, tile)) in &mut game_state.world.query::<(&Unit, &Owner, &Tile)>() {
        if owner.0 == game_state.me.id {
            continue;
        };

        let distance = around.unsigned_distance_to(tile.hex);
        if distance >= lowest_distance {
            continue;
        }

        closest_enemy_hex = Some(tile.hex);
        lowest_distance = distance;
    }

    closest_enemy_hex
}

fn find_enemy_hexes_in_range(game_state: &BotGameState, around: Hex, range: u32) -> Vec<Hex> {
    let mut enemy_hexes = find_enemy_hexes(game_state);

    for hex in shapes::hexagon(around, range) {
        let Some(entity) = game_state.map.entity_at(&hex, GameObjectKind::Unit) else {
            continue;
        };
        let mut query = game_state
            .world
            .query_one::<(&Unit, &Owner)>(*entity)
            .unwrap();
        let Some((unit, owner)) = query.get() else {
            continue;
        };

        if owner.0 == game_state.me.id {
            continue;
        };

        enemy_hexes.push(hex);
    }

    enemy_hexes
}

fn attack_enemy(game_state: &BotGameState, unit_hex: Hex, enemy_hex: Hex, intents: &mut Intents) {
    // decide wether to attack based on current energy, shield health, and move needs

    //

    intents.push(Intent::UnitAttack(UnitAttack {
        attacker_hex: unit_hex,
        target_hex: enemy_hex,
        target_kind: GameObjectKind::Unit,
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

        if let Some(terrain_entity) = game_state.map.entity_at(&bhex, GameObjectKind::Terrain) {
            game_state
                .world
                .query_one::<Or<&Lava, &Wall>>(*terrain_entity)
                .ok()?
                .get()?;
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
        println!("[generalist ai] no path found");
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
    // loop through turrets
    // shoot at closest enemy

    for (turret_entity, (turret, turret_tile, turret_owner)) in
        game_state.world.query::<(&Turret, &Tile, &Owner)>().iter()
    {
        if turret_owner.0 != game_state.me.id {
            continue;
        };

        for (unit_entity, (unit, unit_tile, unit_owner)) in
            game_state.world.query::<(&Unit, &Tile, &Owner)>().iter()
        {
            if unit_owner.0 != game_state.me.id {
                continue;
            };
        }
    }
}

pub fn factories_spawn_units(
    game_state: &BotGameState,
    memory: &mut BotMemory,
    intents: &mut Intents,
) {
    for (entity, (factory, tile, owner)) in
        game_state.world.query::<(&Factory, &Tile, &Owner)>().iter()
    {
        if owner.0 != game_state.me.id {
            continue;
        };

        println!(
            "[generalist ai] trying to spawn a unit at ({}, {})",
            tile.hex.x, tile.hex.y
        );

        let parts = vec![
            (UnitPart::Generate, 5),
            (UnitPart::Ranged, 1),
            (UnitPart::Shield, 1),
        ];

        intents.push(Intent::FactorySpawnUnit(FactorySpawnUnit {
            factory_hex: tile.hex,
            out: None,
            name: "attacker".to_string(),
            body: UnitBody::from_vec(parts),
            owner: owner.0,
        }));
    }
}

use ashscript_types::{components::{energy::Energy, factory::Factory, health::Health, owner::Owner, storage::Storage, tile::Tile, turret::Turret}, objects::GameObjectKind, player::PlayerId};
use hecs::Entity;
use hexx::Hex;

use crate::game_state::GameState;

pub fn spawn_turret(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    let entity = game_state.world.spawn((GameObjectKind::Turret, Turret::default(), Tile::new(hex), Owner(owner_id), Energy::for_structure(&GameObjectKind::Turret), Health::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Turret].insert(hex, entity);

    entity
}

pub fn spawn_factory(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    let entity = game_state.world.spawn((GameObjectKind::Factory, Factory::default(), Tile::new(hex), Owner(owner_id), Storage::default(), Health::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Factory].insert(hex, entity);

    entity
}

pub fn spawn_turbine(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    let entity = game_state.world.spawn((GameObjectKind::Turbine, Tile::new(hex), Owner(owner_id), Health::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Turbine].insert(hex, entity);

    entity
}

pub fn spawn_solar_panel(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    let entity = game_state.world.spawn((GameObjectKind::SolarPanel, Tile::new(hex), Owner(owner_id), Health::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::SolarPanel].insert(hex, entity);

    entity
}

pub fn spawn_substation(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    let entity = game_state.world.spawn((GameObjectKind::Substation, Tile::new(hex), Owner(owner_id), Energy::for_structure(&GameObjectKind::Substation), Health::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Substation].insert(hex, entity);

    entity
}
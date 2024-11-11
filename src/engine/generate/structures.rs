use ashscript_types::{components::{energy::Energy, factory::Factory, health::Health, owner::Owner, storage::Storage, tile::Tile, turret::Turret}, objects::GameObjectKind, player::PlayerId};
use hecs::Entity;
use hexx::Hex;

use crate::game_state::GameState;

pub fn spawn_turret(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    let entity = game_state.world.spawn((GameObjectKind::Terrain, Turret::default(), Tile::new(hex), Owner(owner_id), Energy::default(), Health::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Factory].insert(hex, entity);

    entity
}

pub fn spawn_factory(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) -> Entity {
    let entity = game_state.world.spawn((GameObjectKind::Factory, Factory::default(), Tile::new(hex), Owner(owner_id), Storage::default(), Health::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Factory].insert(hex, entity);

    entity
}
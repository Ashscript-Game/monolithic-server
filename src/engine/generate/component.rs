use ashscript_types::{components::{body::{UnitBody, UnitPart}, energy::Energy, health::Health, owner::Owner, storage::Storage, tile::Tile, unit::Unit}, objects::GameObjectKind, player::PlayerId};
use hecs::World;
use hexx::Hex;

use crate::game_state::GameState;

pub fn new_unit(game_state: &mut GameState, name: String, hex: Hex, body: UnitBody, owner_id: PlayerId) -> hecs::Entity {
    let entity = game_state.world.spawn((Unit::new(name), body, Tile::new(hex), Owner(owner_id), Storage::default(), Health::default(), Energy::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Unit].insert(hex, entity);

    entity
}
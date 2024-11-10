use ashscript_types::{components::{body::{UnitBody, UnitPart}, unit::Unit}, objects::GameObjectKind, player::PlayerId};
use hecs::World;
use hexx::Hex;

use crate::game_state::GameState;

pub fn new_unit(game_state: &mut GameState, name: String, hex: Hex, body: UnitBody, owner_id: PlayerId) -> hecs::Entity {
    let entity = game_state.world.spawn((Unit::default(), UnitBody::default()));
    game_state.map.chunk_at_mut(&hex).unwrap().entities[GameObjectKind::Unit].insert(hex, entity);

    entity
}
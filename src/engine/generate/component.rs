use ashscript_types::{components::body::{UnitBody, UnitPart}, player::PlayerId, unit::Unit};
use hecs::World;
use hexx::Hex;

use crate::game_state::GameState;

pub fn new_unit(game_state: &mut GameState, name: String, hex: Hex, body: UnitBody, owner_id: PlayerId) -> hecs::Entity {
    game_state.world.spawn((Unit::default(), UnitBody::default()))
}
use ashscript_types::{player::PlayerId, unit::Unit};
use hexx::Hex;

use crate::game_state::GameState;

pub fn spawn_unit(game_state: &mut GameState, hex: Hex, owner_id: PlayerId) {
    let Some(chunk) = game_state.map.chunk_at_mut(&hex) else {
        return;
    };

    chunk.units.insert(hex, Unit {
        hex,
        owner_id,
        ..Default::default()
    });
}
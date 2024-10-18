use ashscript_types::objects::GameObjectKind;
use hexx::hex;
use uuid::Uuid;

use crate::{engine::generate::structures::spawn_structure, game_state::GameState};

pub fn generate(game_state: &mut GameState) {

    let player_ids = game_state.global.players.keys().cloned().collect::<Vec<Uuid>>();
    for player_id in player_ids {

        spawn_structure(game_state, hex(8, 6), player_id, GameObjectKind::Factory);
    }
}   
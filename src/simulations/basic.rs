use ashscript_types::{objects::GameObjectKind, player::Player};
use hexx::hex;
use uuid::{uuid, Uuid};

use crate::{engine::generate::structures::spawn_structure, game_state::GameState};

pub fn generate(game_state: &mut GameState) {
    for i in 0..2 {
        let id = Uuid::new_v4();
        game_state.global.players.insert(
            id,
            Player {
                id,
                name: format!("Player {}", i),
                ..Default::default()
            },
        );

        println!("generating player {} with id {}", i, id);
    }

    let factory_hexes = [hex(8, 6), hex(-8, -3)];

    let player_ids = game_state
        .global
        .players
        .keys()
        .cloned()
        .collect::<Vec<Uuid>>();
    for (i, player_id) in player_ids.iter().enumerate() {
        println!("spawning factory for player with id {}", player_id);

        let hex = factory_hexes[i];
        spawn_structure(game_state, hex, *player_id, GameObjectKind::Factory);
    }
}

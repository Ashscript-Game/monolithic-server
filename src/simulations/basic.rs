use ashscript_types::{components::{factory::Factory, owner::Owner, storage::Storage, tile::Tile}, objects::GameObjectKind, player::Player, resource::Resource};
use hexx::hex;
use uuid::Uuid;

use crate::{engine::generate::structures::{spawn_factory, spawn_turret}, game_state::GameState};

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

    let factory_hexes = [hex(14, -6), hex(-8, 4)];
    let turret_hexes = [hex(17, -7), hex(-11, 5)];

    let player_ids = game_state
        .global
        .players
        .keys()
        .cloned()
        .collect::<Vec<Uuid>>();
    for (i, player_id) in player_ids.iter().enumerate() {

        // factories

        let hex = factory_hexes[i];
        spawn_factory(game_state, hex, *player_id);

        let factory_entity = spawn_factory(game_state, hex, *player_id);
        let (_, factory_storage) = game_state.world.query_one_mut::<(&Factory, &mut Storage)>(factory_entity).unwrap();
        factory_storage.capacity = 10_000;

        factory_storage.add_checked(&Resource::Metal, &1000).unwrap();

        // turrets

        let hex = turret_hexes[i];
        spawn_turret(game_state, hex, *player_id);
    }
}
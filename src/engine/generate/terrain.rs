use ashscript_types::components::{
    resource::{CoalNode, MineralNode, ResourceNode},
    terrain::{Terrain, TerrainKind, Wall},
};
use hexx::{hex, shapes};
use libnoise::prelude::*;

use crate::{
    engine::constants::terrain::{resource_noise_tresholds, SIMPLEX_GENERATOR},
    game_state::GameState,
};

pub fn generate_terrain(game_state: &mut GameState) {
    for hex in shapes::hexagon(hex(0, 0), game_state.map.data.radius) {
        let noise = SIMPLEX_GENERATOR.sample([
            hex.x as f64,
            hex.y as f64, /* hex.x as f64, hex.y as f64 */
        ]);

        let Some(chunk) = game_state.map.chunks.get_mut(&hex) else {
            continue;
        };

        if noise > resource_noise_tresholds::WALL.0 && noise < resource_noise_tresholds::WALL.1 {
            game_state.world.spawn((Terrain { kind: TerrainKind::Wall }, Wall));

            continue;
        }

        if noise > resource_noise_tresholds::COAL.0 && noise < resource_noise_tresholds::COAL.1 {
            game_state.world.spawn((CoalNode {}, ResourceNode::new(0)));
            continue;
        }

        if noise > resource_noise_tresholds::MINERALS.0
            && noise < resource_noise_tresholds::MINERALS.1
        {
            game_state.world.spawn((MineralNode {}, ResourceNode::new(0)));
            continue;
        }

        // if noise > resource_noise_tresholds::SCRAP.0 && noise < resource_noise_tresholds::SCRAP.1 {
        //     chunk.scrap.insert(
        //         hex,
        //         Scrap {
        //             ..Default::default()
        //         },
        //     );
        //     continue;
        // }
    }
}

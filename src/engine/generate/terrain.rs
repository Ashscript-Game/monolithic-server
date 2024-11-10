use ashscript_types::{
    components::{
        resource::{CoalNode, MineralNode, ResourceNode},
        terrain::{Lava, Terrain, TerrainKind, Wall},
        tile::Tile,
    },
    constants::map::CHUNK_SIZE,
};
use hexx::{hex, shapes, Hex};
use libnoise::prelude::*;

use crate::{
    engine::constants::terrain::{resource_noise_tresholds, SIMPLEX_GENERATOR},
    game_state::GameState,
};

pub fn generate_terrain(game_state: &mut GameState) {
    for chunk_hex in game_state.map.chunks.keys() {
        for hex in shapes::hexagon(chunk_hex.to_higher_res(CHUNK_SIZE), CHUNK_SIZE) {
            let noise = SIMPLEX_GENERATOR.sample([
                hex.x as f64,
                hex.y as f64, /* hex.x as f64, hex.y as f64 */
            ]);

            if noise > resource_noise_tresholds::WALL.0 && noise < resource_noise_tresholds::WALL.1
            {
                game_state.world.spawn((
                    Terrain {
                        kind: TerrainKind::Wall,
                    },
                    Wall,
                    Tile::new(hex),
                ));

                continue;
            }

            if noise > resource_noise_tresholds::COAL.0 && noise < resource_noise_tresholds::COAL.1
            {
                game_state
                    .world
                    .spawn((CoalNode {}, ResourceNode::new(0), Tile::new(hex)));
                continue;
            }

            if noise > resource_noise_tresholds::MINERALS.0
                && noise < resource_noise_tresholds::MINERALS.1
            {
                game_state
                    .world
                    .spawn((MineralNode {}, ResourceNode::new(0), Tile::new(hex)));
                continue;
            }

            if noise > resource_noise_tresholds::LAVA.0 && noise < resource_noise_tresholds::LAVA.1
            {
                game_state.world.spawn((
                    Terrain {
                        kind: TerrainKind::Lava,
                    },
                    Lava,
                    Tile::new(hex),
                ));
                continue;
            }
        }
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

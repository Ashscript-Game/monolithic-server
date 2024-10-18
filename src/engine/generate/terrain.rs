use ashscript_types::terrain::{CoalNode, MineralNode, Scrap, Terrain};
use hexx::{hex, shapes};
use libnoise::prelude::*;

use crate::{
    engine::constants::terrain::{resource_noise_tresholds, SIMPLEX_GENERATOR},
    game_state::GameState,
};

pub fn generate_terrain(game_state: &mut GameState) {
    for hex in shapes::hexagon(hex(0, 0), game_state.map.radius) {
        let noise = SIMPLEX_GENERATOR.sample([
            hex.x as f64,
            hex.y as f64, /* hex.x as f64, hex.y as f64 */
        ]);

        let Some(chunk) = game_state.map.chunks.get_mut(&hex) else {
            continue;
        };

        if noise > resource_noise_tresholds::WALL.0 && noise < resource_noise_tresholds::WALL.1 {
            chunk.terrain.insert(hex, Terrain::Wall);

            continue;
        }

        if noise > resource_noise_tresholds::COAL.0 && noise < resource_noise_tresholds::COAL.1 {
            chunk.coal_nodes.insert(hex, CoalNode {
                ..Default::default()
            });
            continue;
        }

        if noise > resource_noise_tresholds::MINERALS.0
            && noise < resource_noise_tresholds::MINERALS.1
        {
            chunk.mineral_nodes.insert(hex, MineralNode {
                ..Default::default()
            });
            continue;
        }

        if noise > resource_noise_tresholds::SCRAP.0 && noise < resource_noise_tresholds::SCRAP.1 {
            chunk.scrap.insert(hex, Scrap {
                ..Default::default()
            });
            continue;
        }
    }
}

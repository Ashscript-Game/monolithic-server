use ashscript_types::{
    chunk::Chunk,
    components::{
        resource::{CoalNode, MineralNode, ResourceNode},
        terrain::{Lava, Terrain, TerrainKind, Wall},
        tile::Tile,
    },
    constants::map::CHUNK_SIZE,
    objects::GameObjectKind, resource::Resource,
};
use hecs::{Entity, World};
use hexx::{hex, shapes, Hex};
use libnoise::prelude::*;

use crate::{
    engine::constants::terrain::{resource_noise_tresholds, SIMPLEX_GENERATOR},
    game_state::GameState,
};

pub fn generate_terrain(game_state: &mut GameState) {
    for (chunk_hex, chunk) in game_state.map.chunks.iter_mut() {
        for hex in shapes::hexagon(chunk_hex.to_higher_res(CHUNK_SIZE), CHUNK_SIZE) {
            let noise = SIMPLEX_GENERATOR.sample([
                hex.x as f64,
                hex.y as f64, /* hex.x as f64, hex.y as f64 */
            ]);

            if noise > resource_noise_tresholds::WALL.0 && noise < resource_noise_tresholds::WALL.1
            {
                spawn_wall_entity(&mut game_state.world, hex, chunk);
                continue;
            }

            if noise > resource_noise_tresholds::COAL.0 && noise < resource_noise_tresholds::COAL.1
            {
                let entity = game_state.world.spawn((
                    GameObjectKind::ResourceNode,
                    CoalNode {},
                    ResourceNode::new(0, Resource::Coal),
                    Tile::new(hex),
                ));
                chunk.entities[GameObjectKind::Terrain].insert(hex, entity);

                spawn_wall_entity(&mut game_state.world, hex, chunk);
                continue;
            }

            if noise > resource_noise_tresholds::MINERALS.0
                && noise < resource_noise_tresholds::MINERALS.1
            {
                let entity = game_state.world.spawn((
                    GameObjectKind::ResourceNode,
                    MineralNode {},
                    ResourceNode::new(0, Resource::Minerals),
                    Tile::new(hex),
                ));
                chunk.entities[GameObjectKind::Terrain].insert(hex, entity);

                spawn_wall_entity(&mut game_state.world, hex, chunk);
                continue;
            }

            if noise > resource_noise_tresholds::LAVA.0 && noise < resource_noise_tresholds::LAVA.1
            {
                let entity = game_state.world.spawn((
                    GameObjectKind::Terrain,
                    Terrain {
                        kind: TerrainKind::Lava,
                    },
                    Lava,
                    Tile::new(hex),
                ));
                chunk.entities[GameObjectKind::Terrain].insert(hex, entity);
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

fn spawn_wall_entity(world: &mut World, hex: Hex, chunk: &mut Chunk) {
    let entity = world.spawn((
        GameObjectKind::Terrain,
        Terrain {
            kind: TerrainKind::Wall,
        },
        Wall,
        Tile::new(hex),
    ));
    chunk.entities[GameObjectKind::Terrain].insert(hex, entity);
}

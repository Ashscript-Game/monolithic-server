use crate::{engine::constants::terrain::SIMPLEX_GENERATOR, game_state::GameState};
use ashscript_types::{chunk::Chunk, constants::map::CHUNK_SIZE};
use hexx::{hex, shapes, Hex};
use libnoise::Generator;

// generate tiles for a map
pub fn generate_tiles(game_state: &mut GameState) {
    for hex in shapes::hexagon(hex(0, 0), game_state.map.data.radius) {

        if game_state.map.chunk_at(&hex).is_some() {
            // println!("skipping existing chunk for hex {}, {}", hex.x, hex.y);
            continue;
        };

        // Otherwise no chunk exists for this hex. Create one

        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        let noise = SIMPLEX_GENERATOR.sample([
            chunk_hex.x as f64,
            chunk_hex.y as f64, /* hex.x as f64, hex.y as f64 */
        ]);
        game_state.map.chunks.insert(chunk_hex, Chunk::new(chunk_hex, noise));
    }
}

// Add tiles to an already populated map
pub fn expand_tiles(game_state: &mut GameState) {}
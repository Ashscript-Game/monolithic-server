use crate::game_state::GameState;
use ashscript_types::{chunk::Chunk, constants::map::CHUNK_SIZE};
use hexx::{hex, shapes, Hex};

// generate tiles for a map
pub fn generate_tiles(game_state: &mut GameState) {
    for hex in shapes::hexagon(hex(0, 0), game_state.map.data.radius) {

        if game_state.map.chunk_at(&hex).is_some() {
            // println!("skipping existing chunk for hex {}, {}", hex.x, hex.y);
            continue;
        };

        // Otherwise no chunk exists for this hex. Create one

        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        game_state.map.chunks.insert(chunk_hex, Chunk::new(chunk_hex));
    }
}

// Add tiles to an already populated map
pub fn expand_tiles(game_state: &mut GameState) {}
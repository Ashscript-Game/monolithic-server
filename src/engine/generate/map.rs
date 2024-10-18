use crate::game_state::GameState;
use ashscript_types::{chunk::Chunk, constants::map::CHUNK_SIZE};
use hexx::{hex, shapes};

// generate tiles for a map
pub fn generate_tiles(game_state: &mut GameState) {
    for hex in shapes::hexagon(hex(0, 0), game_state.map.radius) {

        println!("generating for radius {}", game_state.map.radius);

        if game_state.map.chunk_at(&hex).is_some() {

            println!("skipping existing chunk for hex {}, {}", hex.x , hex.y);
            continue;
        };

        println!("generating new chunk for hex {}, {}", hex.x , hex.y);

        // Otherwise no chunk exists for this hex. Create one

        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        game_state.map.chunks.insert(chunk_hex, Chunk::new());
    }
}

// Add tiles to an already populated map
pub fn expand_tiles(game_state: &mut GameState) {

}
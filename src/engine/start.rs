use crate::{game_state::GameState, simulations};

use super::{generate::{map::generate_tiles, terrain::generate_terrain}, runner::runner};

pub async fn start() {
    let mut game_state = GameState::new();
    game_state.map.radius = 100;

    generate_tiles(&mut game_state);
    generate_terrain(&mut game_state);

    simulations::basic::generate(&mut game_state);

    runner(&mut game_state).await;
}
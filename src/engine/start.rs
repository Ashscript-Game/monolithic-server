use crate::game_state::GameState;

use super::{generate::{map::generate_tiles, terrain::generate_terrain}, runner::runner};

pub async fn start() {
    let mut game_state = GameState::new();

    generate_tiles(&mut game_state);
    generate_terrain(&mut game_state);

    runner(&mut game_state).await;
}
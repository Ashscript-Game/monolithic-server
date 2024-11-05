use super::{
    generate::{map::generate_tiles, terrain::generate_terrain},
    runner::runner,
};
use crate::{game_state::GameState, simulations};
use std::sync::Arc;
use tokio::sync::broadcast::Sender;

pub async fn start(sender: Sender<Arc<Vec<u8>>>) {
    let mut game_state = GameState::new();
    game_state.map.data.radius = 100;

    generate_tiles(&mut game_state);
    generate_terrain(&mut game_state);

    simulations::basic::generate(&mut game_state);

    runner(&mut game_state, sender).await;
}
use std::io;

use socketioxide::SocketIo;

use crate::{game_state::{BotGameState, GameState}, simulations};

use super::{client, generate::{map::generate_tiles, terrain::generate_terrain}, runner::runner};

pub async fn start(io: &SocketIo) {
    let mut game_state = GameState::new();
    game_state.map.radius = 100;

    generate_tiles(&mut game_state);
    generate_terrain(&mut game_state);

    simulations::basic::generate(&mut game_state);

    io.ns("/client", client::on_connect);

    runner(&mut game_state, io).await;
}
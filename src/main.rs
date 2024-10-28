use std::future::poll_fn;

use ashscript_types::intents::Intents;
use axum::routing::get;
use engine::{runner::runner, start::start};
use game_state::{BotGameState, GameState};
use log::info;
use logging::setup_logger;
use socketioxide::SocketIo;

pub mod ai;
pub mod engine;
pub mod game_state;
pub mod logging;
pub mod simulations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    info!("Welcome to the AshScript monolithic server. Starting web-services.");

    let (layer, io) = SocketIo::builder()
        .with_state(BotGameState::new(&GameState::new()))
        .build_layer();

    let app = axum::Router::new()
        .route(
            "/",
            get(move || async move {
                start(&io).await;
                "AshScript Monolith."
            }),
        )
        .layer(layer);

    info!("Starting axum / socketio server.");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let _ = axum::serve(listener, app).await?;

    // start(&io).await;

    Ok(())
}

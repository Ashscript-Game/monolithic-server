use ashscript_types::intents::Intents;
use axum::routing::get;
use engine::{runner::runner, start::start};
use game_state::GameState;
use log::info;
use logging::setup_logger;
use socketioxide::SocketIo;

pub mod client;
pub mod logging;
pub mod engine;
pub mod game_state;
pub mod ai;
pub mod simulations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    info!("Welcome to the AshScript monolithic server. Starting web-services.");

    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/client", client::on_connect);

    // let app = axum::Router::new()
    //     .route("/", get(|| async { "AshScript Monolith." }))
    //     .layer(layer);

    // info!("Starting axum / socketio server.");

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    // axum::serve(listener, app).await?;

    start().await;

    Ok(())
}
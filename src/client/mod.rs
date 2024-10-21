use std::time::Duration;

use log::info;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use tokio::time::sleep;

use crate::game_state::{BotGameState, GameState};

pub async fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(
        "New socket connection from: {} - id: {}",
        socket.ns(),
        socket.id
    );

    let game_state = GameState::new();

    loop {
        socket
            .emit(
                "game",
                serde_json::json!({
                    "map": game_state.map,
                    "global": game_state.global,
                })
            )
            .unwrap();

        sleep(Duration::from_secs(1)).await;
    }

    // socket.emit("keyframe", json!({
    //     "chunk": 1,
    //     "units": ["epicUnit1", "epicUnit2", "KillMeNow."],
    //     "structures": [{
    //         "id": "epicStructure1",
    //         "type": "spawn",
    //         "position": [40, 20]
    //     }, {
    //         "id": "epicStructure2",
    //         "type": "spawn",
    //         "position": [20, 25]
    //     }]
    // })).unwrap();

    // loop {
    //     socket.emit("action", json!({
    //         "actions": [{
    //             "id": "epicUnit1",
    //             "move": 1,
    //         }]
    //     })).unwrap();

    //     sleep(Duration::from_secs(1)).await;
    // }
}

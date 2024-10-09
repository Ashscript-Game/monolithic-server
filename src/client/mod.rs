use std::{collections, time::Duration};

use log::info;
use serde_json::{json, Value};
use socketioxide::extract::{Data, SocketRef};
use tokio::time::sleep;

use crate::game_state::GameState;

pub async fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(
        "New socket connection from: {} - id: {}",
        socket.ns(),
        socket.id
    );

    let mut game_state = GameState {
        units: collections::HashMap::new(),
        turret: collections::HashMap::new(),
    };

    loop {
        socket
            .emit(
                "keyframe",
                serde_json::from_value(game_state)
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

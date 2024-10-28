use std::time::Duration;

use ashscript_types::{keyframe::KeyFrame, map::Map};
use hashbrown::HashMap;
use hexx::{hex, Hex};
use log::info;
use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::time::sleep;

use crate::game_state::{BotGameState, GameState};

pub async fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(
        "New socket connection from: {} - id: {}",
        socket.ns(),
        socket.id
    );

    let game_state = GameState::new();

    /* loop {
        socket
            .emit(
                "game_state",
                serde_json::json!({
                    "map": game_state.map,
                    "global": game_state.global,
                })
            )
            .unwrap();

        println!("sent game state to client");

        sleep(Duration::from_secs(1)).await;
    } */

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

pub fn emit_tick(game_state: &GameState, io: &SocketIo) {
    /* let mut map: HashMap<Hex, Hex> = HashMap::new();
    map.insert(hex(0, 0), hex(0, 0));

    let dat = postcard::to_stdvec(&map).expect("failed to postcard serialize");
    println!("data {:?}", dat.as_slice()); */

    let ser_map = postcard::to_stdvec(&game_state.map).expect("failed to postcard map");
    println!("ser map: {}", ser_map.len());

    let ser_global = postcard::to_stdvec(&game_state.global).expect("failed to postcard global");
    println!("ser global: {}", ser_global.len());

    let keyframe = KeyFrame::from_existing(game_state.map.clone(), game_state.global.clone());

    let ser_keyframe = postcard::to_stdvec(&keyframe).expect("failed to postcard keyframe");
    println!("ser keyframe len: {}", ser_keyframe.len());

    // Emit the actions that have happened since

    io.of("/client")
        .unwrap()
        .emit(
            "game_state",
            ser_keyframe,
            
            /* serde_json::json!({
                "map": game_state.map,
                "global": game_state.global,
                "hash": map,
            } */
        )
        .unwrap();

    println!("tick emit");
}

pub fn basic_emit(game_state: &mut GameState, io: &SocketIo) {
    io.of("/client")
        .unwrap()
        .emit(
            "game_state",
            serde_json::json!({
                "key": "value"
                /* "map": game_state.map,
                "global": game_state.global, */
            }),
        )
        .unwrap();

    println!("basic emit");
}

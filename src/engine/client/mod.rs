use crate::game_state::GameState;
use ashscript_types::keyframe::KeyFrame;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast::Sender;
//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use log::{debug, error, info};
use serde_json::Value;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::{
        connect,
        handshake::server::{Request, Response},
    },
};

pub fn emit_tick(game_state: &GameState, sender: &mut Sender<Arc<Vec<u8>>>) {
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

    match sender.send(Arc::new(ser_keyframe)) {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }

    println!("tick emit");
}

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    receiver: tokio::sync::broadcast::Receiver<Arc<Vec<u8>>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, receiver))
}
/*
async fn server() {
    let server = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    while let Ok((stream, _)) = server.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let callback = |req: &Request, mut response: Response| {
        debug!("Received a new ws handshake");
        debug!("The request's path is: {}", req.uri().path());
        debug!("The request's headers are:");
        for (ref header, _value) in req.headers() {
            debug!("* {}: {:?}", header, _value);
        }

        let headers = response.headers_mut();
        headers.append("MyCustomHeader", ":)".parse().unwrap());

        Ok(response)
    };
    let mut ws_stream = accept_hdr_async(stream, callback)
        .await
        .expect("Error during the websocket handshake occurred");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg.unwrap();
        if msg.is_text() || msg.is_binary() {
            debug!("Server on message: {:?}", &msg);
            ws_stream.send(msg).await.unwrap();
        }
    }
}

fn client() {
    let (mut socket, response) = connect("ws://localhost:8080/socket").expect("Can't connect");
    debug!("Connected to the server");
    debug!("Response HTTP code: {}", response.status());
    debug!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        debug!("* {}: {:?}", header, _value);
    }

    socket
        .send(Message::Text("Hello WebSocket".into()))
        .unwrap();
    loop {
        let msg = socket.read().expect("Error reading message");
        debug!("Received: {}", msg);
    }
}
 */

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    mut socket: WebSocket,
    mut receiver: tokio::sync::broadcast::Receiver<Arc<Vec<u8>>>,
) {
    loop {
        let value = receiver.recv().await;
        let value = match value {
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
            Ok(v) => v,
        };

        // UGH WHY DOES THIS NEED A FULL VEC
        match socket.send(Message::Binary((*value).clone())).await {
            Err(e) => println!("{:?}", e),
            Ok(_) => {}
        }
    }
}

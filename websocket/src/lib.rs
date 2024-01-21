mod error;
mod game_action;
mod handler;
mod helper;
mod lobby;
mod network;
mod payload;
mod response;

use crate::handler::handle;
use crate::lobby::Lobby;
use axum::extract::ws::Message;
use axum::routing::get;
use axum::Router;
use game::Game;
use std::collections::HashMap;
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, RwLock};

pub async fn start_game_server() {
    let state = Arc::new(WebSocketState::new());
    let app = Router::new().route("/ws", get(handle)).with_state(state);

    println!("starting...");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

struct WebSocketState {
    games: RwLock<HashMap<String, Game>>,
    lobbies: RwLock<HashMap<String, Lobby>>,
    player_connections: RwLock<HashMap<String, mpsc::Sender<Message>>>,
    broadcast_sender: broadcast::Sender<Message>,
}

impl WebSocketState {
    pub fn new() -> WebSocketState {
        WebSocketState {
            games: RwLock::new(HashMap::new()),
            lobbies: RwLock::new(HashMap::new()),
            player_connections: RwLock::new(HashMap::new()),
            broadcast_sender: broadcast::channel::<Message>(128).0,
        }
    }
}

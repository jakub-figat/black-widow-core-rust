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
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};

pub async fn start_game_server() {
    let state = Arc::new(WebSocketGameState::new());
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

struct WebSocketGameState {
    games: RwLock<HashMap<String, Game>>,
    lobbies: RwLock<HashMap<String, Lobby>>,
    broadcast_sender: broadcast::Sender<Message>,
}

impl WebSocketGameState {
    pub fn new() -> WebSocketGameState {
        WebSocketGameState {
            games: RwLock::new(HashMap::new()),
            lobbies: RwLock::new(HashMap::new()),
            broadcast_sender: broadcast::channel::<Message>(128).0,
        }
    }
}

mod error;
mod game_action;
mod handler;
mod helper;
mod lobby;
mod network;
pub mod payload;
pub mod response;

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
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};
use uuid::Uuid;

pub async fn start_game_server() {
    let state = Arc::new(WebSocketState::new());
    let app = Router::new().route("/ws", get(handle)).with_state(state);

    println!("Starting server on port 6379");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

struct WebSocketState {
    games: Mutex<HashMap<Uuid, Game>>,
    lobbies: Mutex<HashMap<Uuid, Lobby>>,
    player_connections: RwLock<HashMap<String, mpsc::Sender<Message>>>,
    broadcast_sender: broadcast::Sender<Message>,
}

impl WebSocketState {
    pub fn new() -> WebSocketState {
        WebSocketState {
            games: Mutex::new(HashMap::new()),
            lobbies: Mutex::new(HashMap::new()),
            player_connections: RwLock::new(HashMap::new()),
            broadcast_sender: broadcast::channel::<Message>(128).0,
        }
    }
}

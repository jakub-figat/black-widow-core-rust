mod payload;
mod game_action;
mod handler;
mod response;
mod helper;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::ws::Message;
use axum::{Router};
use axum::routing::get;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};
use game::Game;
use crate::handler::handle;


pub async fn start_game_server() {
    let state = Arc::new(WebSocketGameState::new());
    let app = Router::new()
        .route("/ws", get(handle))
        .with_state(state);

    println!("starting...");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}


#[derive(Debug)]
struct WebSocketGameState {
    games: RwLock<HashMap<String, Game>>,
    broadcast_sender: broadcast::Sender<Message>
}

impl WebSocketGameState {
    pub fn new() -> WebSocketGameState {
        WebSocketGameState {
            games: RwLock::new(HashMap::new()),
            broadcast_sender: broadcast::channel::<Message>(128).0
        }
    }
}

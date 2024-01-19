use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::Arc;
use axum::extract::ws::Message;
use tokio::sync::broadcast;
use game::{Game, GameSettings};
use crate::WebSocketGameState;
use crate::handler::broadcast_text_or_break;

pub(crate) async fn start_game(
    broadcast_sender: &mut broadcast::Sender<Message>,
    state: Arc<WebSocketGameState>,
    address: &SocketAddr
) -> ControlFlow<(), ()> {
    let game = Game::new_by_player(
        &address.to_string(), GameSettings {max_score: 100}
    );
    state.games.write().await.insert("some_uid4".to_string(), game);

    // broadcast to all
    // TODO
    broadcast_text_or_break("game started", broadcast_sender)?;
    ControlFlow::Continue(())
}

// TODO: many more game actions
// listing games on demand, quitting, joining
// broadcasting when necessary, obfuscate what necessary
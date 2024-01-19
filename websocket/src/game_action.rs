use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::Arc;
use axum::extract::ws::Message;
use tokio::sync::{broadcast, mpsc};
use game::{Game, GameSettings};
use crate::WebSocketGameState;
use crate::handler::{broadcast_text_or_break, send_text_or_break};
use crate::response::GameListResponse;

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

pub(crate) async fn list_games(
    sender: &mut mpsc::Sender<Message>,
    state: Arc<WebSocketGameState>
) -> ControlFlow<(), ()> {
    let game_hashmap = state.games.read().await;
    let response = GameListResponse::json_from_game_hashmap(&game_hashmap);

    send_text_or_break(&response, sender).await
}


// TODO: check if player belongs to given game
pub(crate) async fn get_game_details() {

}

// TODO:
// create (&join game)
// join game
// get game detail


// game moves

// cards for exchange
// place card
// claim readiness
// quit


// ws auth
// maybe redis for shared state if scaling instances
// broadcasting when necessary, obfuscate what necessary
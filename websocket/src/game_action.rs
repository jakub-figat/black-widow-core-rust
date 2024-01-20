use crate::helper::{broadcast_text_or_break, send_text_or_break};
use crate::response::{
    CardExchangeState, ErrorResponse, GameDetails, GameListResponse, RoundFinishedState,
    RoundInProgressState,
};
use crate::WebSocketGameState;
use axum::extract::ws::Message;
use game::{CardExchange, RoundFinished, RoundInProgress};
use std::ops;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

type ControlFlow = ops::ControlFlow<(), ()>;
type Sender = mpsc::Sender<Message>;
type BroadcastSender = broadcast::Sender<Message>;

pub(crate) async fn list_lobbies(
    sender: &mut Sender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    ControlFlow::Continue(())
}

pub(crate) async fn get_lobby_details(
    id: &str,
    sender: &mut Sender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    ControlFlow::Continue(())
}

pub(crate) async fn create_lobby(
    player: &str,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    broadcast_text_or_break("gitara siema", broadcast_sender)
}

pub(crate) async fn join_lobby(
    id: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    ControlFlow::Continue(())
}

pub(crate) async fn quit_lobby(
    id: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    ControlFlow::Continue(())
}

pub(crate) async fn list_games(sender: &mut Sender, state: Arc<WebSocketGameState>) -> ControlFlow {
    let game_hashmap = state.games.read().await;
    let response = GameListResponse::json_from_game_hashmap(&game_hashmap);
    send_text_or_break(&response, sender).await
}

pub(crate) async fn get_game_details(
    id: &str,
    player: &str,
    sender: &mut Sender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    let game_hashmap = state.games.read().await;
    let response = match game_hashmap.get(id) {
        Some(game) => match game.players.iter().find(|&s| s.as_str() == player) {
            Some(_) => match game.state.as_ref().unwrap() {
                CardExchange(_) => GameDetails::<CardExchangeState>::json_from_game(game, player),
                RoundInProgress(_) => {
                    GameDetails::<RoundInProgressState>::json_from_game(game, player)
                }
                RoundFinished(_) => GameDetails::<RoundFinishedState>::json_from_game(game, player),
            },
            None => {
                ErrorResponse::json_from_detail(&format!("You don't belong to game with id {}", id))
            }
        },
        None => ErrorResponse::json_from_detail(&format!("Game with id {} does not exist", id)),
    };
    send_text_or_break(&response, sender).await
}

pub(crate) async fn game_move(
    payload: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    ControlFlow::Continue(())
}

pub(crate) async fn quit_game(
    id: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    ControlFlow::Continue(())
}

// TODO:
// add lobbies, without them sending game state is nightmare
// ws auth
// maybe redis for shared state if scaling instances

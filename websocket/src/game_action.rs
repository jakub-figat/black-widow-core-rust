use crate::lobby::Lobby;
use crate::network::{broadcast_text_or_break, send_error_or_break, send_text_or_break};
use crate::payload::MaxPlayersPayload;
use crate::response::{
    CardExchangeState, ErrorResponse, GameDetailsResponse, GameListResponse, LobbyDetailsResponse,
    LobbyListResponse, RoundFinishedState, RoundInProgressState, ToJson,
};
use crate::WebSocketGameState;
use axum::extract::ws::Message;
use game::{CardExchange, RoundFinished, RoundInProgress};
use std::ops;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

type ControlFlow = ops::ControlFlow<(), ()>;
type Sender = mpsc::Sender<Message>;
type BroadcastSender = broadcast::Sender<Message>;

pub(crate) async fn list_lobbies(
    sender: &mut Sender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    let lobbies = state.lobbies.read().await;
    let response = LobbyListResponse::new(&lobbies);
    send_text_or_break(&response.to_json(), sender).await;
    ControlFlow::Continue(())
}

pub(crate) async fn get_lobby_details(
    id: &str,
    sender: &mut Sender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    let lobbies = state.lobbies.read().await;
    match lobbies.get(id) {
        Some(lobby) => {
            let response = LobbyDetailsResponse::new(lobby);
            send_text_or_break(&response.to_json(), sender).await
        }
        None => send_error_or_break(&format!("Lobby with id {} not found", &id), sender).await,
    }
}

pub(crate) async fn create_lobby(
    payload: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    match serde_json::from_str::<MaxPlayersPayload>(payload) {
        Ok(max_players_payload) => {
            match Lobby::new_by_player(max_players_payload.max_players, player) {
                Ok(lobby) => {
                    let mut lobbies = state.lobbies.write().await;
                    lobbies.insert(Uuid::new_v4().to_string(), lobby.clone());
                    let response = LobbyDetailsResponse::new(&lobby);
                    broadcast_text_or_break(&response.to_json(), broadcast_sender)
                }
                Err(error) => send_error_or_break(&error.to_string(), sender).await,
            }
        }
        Err(error) => send_error_or_break(&error.to_string(), sender).await,
    }
}

pub(crate) async fn join_lobby(
    id: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketGameState>,
) -> ControlFlow {
    let mut lobbies = state.lobbies.write().await;
    match lobbies.get_mut(id) {
        Some(lobby) => {
            lobby.players.push(player.to_string());
            if lobby.players.len() == lobby.max_players {}
            // TODO: some game magic required
            // theres a need to store player: channel mapping to selectively broadcast

            let response = LobbyDetailsResponse::new(&lobby);
            broadcast_text_or_break(&response.to_json(), broadcast_sender)
        }
        None => send_error_or_break(&format!("Lobby with id {} not found", &id), sender).await,
    }
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
                CardExchange(_) => {
                    GameDetailsResponse::<CardExchangeState>::json_from_game(id, game, player)
                }
                RoundInProgress(_) => {
                    GameDetailsResponse::<RoundInProgressState>::json_from_game(id, game, player)
                }
                RoundFinished(_) => {
                    GameDetailsResponse::<RoundFinishedState>::json_from_game(id, game, player)
                }
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

// TODO: maybe redis for shared state if scaling instances

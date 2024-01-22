use crate::lobby::Lobby;
use crate::network::{
    broadcast_game_to_players_or_break, broadcast_text_or_break, send_error_or_break,
    send_text_or_break,
};
use crate::payload::{CreateLobbyPayload, GameMovePayload};
use crate::response::{
    game_to_json, GameListResponse, IdResponse, LobbyDetailsResponse, LobbyListResponse,
    WebSocketResponse::*,
};
use crate::WebSocketState;
use axum::extract::ws::Message;
use game::{Game, GameSettings};
use std::ops;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

type ControlFlow = ops::ControlFlow<(), ()>;
type Sender = mpsc::Sender<Message>;
type BroadcastSender = broadcast::Sender<Message>;

pub(crate) async fn list_lobbies(sender: &mut Sender, state: Arc<WebSocketState>) -> ControlFlow {
    let lobbies = state.lobbies.lock().await;
    let response = LobbyList(LobbyListResponse {
        lobbies: lobbies.clone(),
    });
    send_text_or_break(&response.to_json(), sender).await;
    ControlFlow::Continue(())
}

pub(crate) async fn get_lobby_details(
    id: &str,
    sender: &mut Sender,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    let lobbies = state.lobbies.lock().await;
    match lobbies.get(id) {
        Some(lobby) => {
            let response = LobbyDetails(LobbyDetailsResponse {
                lobby: lobby.clone(),
            });
            send_text_or_break(&response.to_json(), sender).await
        }
        None => send_error_or_break(&format!("Lobby with id {} not found", &id), sender).await,
    }
}

pub(crate) async fn create_lobby(
    payload: &CreateLobbyPayload,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    match Lobby::new_by_player(payload.max_players, payload.max_score, player) {
        Ok(lobby) => {
            let mut lobbies = state.lobbies.lock().await;
            lobbies.insert(Uuid::new_v4().to_string(), lobby.clone());

            let response = LobbyDetails(LobbyDetailsResponse {
                lobby: lobby.clone(),
            });
            broadcast_text_or_break(&response.to_json(), broadcast_sender)
        }
        Err(error) => send_error_or_break(&error.to_string(), sender).await,
    }
}

pub(crate) async fn join_lobby(
    id: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    let mut lobbies = state.lobbies.lock().await;

    match lobbies.get_mut(id) {
        Some(lobby) => match add_player_to_lobby(lobby, player, state.clone()).await {
            Some((game_id, game)) => {
                lobbies.remove(id);

                broadcast_text_or_break(
                    &LobbyDeleted(IdResponse { id: id.to_string() }).to_json(),
                    broadcast_sender,
                )?;
                broadcast_game_to_players_or_break(&game_id, &game, state.clone()).await
            }
            None => {
                let response = LobbyDetails(LobbyDetailsResponse {
                    lobby: lobby.clone(),
                });
                broadcast_text_or_break(&response.to_json(), broadcast_sender)
            }
        },
        None => send_error_or_break(&format!("Lobby with id {} not found", &id), sender).await,
    }
}

async fn add_player_to_lobby(
    lobby: &mut Lobby,
    player: &str,
    state: Arc<WebSocketState>,
) -> Option<(String, Game)> {
    lobby.players.push(player.to_string());
    if lobby.players.len() == lobby.max_players {
        let game_id = Uuid::new_v4().to_string();
        let game = Game::from_players(
            &lobby.players,
            GameSettings {
                max_score: lobby.max_score,
            },
        )
        .unwrap();

        state
            .games
            .lock()
            .await
            .insert(game_id.clone(), game.clone());
        Some((game_id, game))
    } else {
        None
    }
}

pub(crate) async fn quit_lobby(
    id: &str,
    player: &str,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    let player = player.to_string();
    let mut lobbies = state.lobbies.lock().await;
    match lobbies.get_mut(id) {
        Some(mut lobby) => {
            if !lobby.players.contains(&player) {
                return send_error_or_break(
                    &format!("You don't belong to lobby with id {}", &id),
                    sender,
                )
                .await;
            }

            let response = match remove_player_from_lobby(player, &mut lobby).await {
                Some(_) => {
                    lobbies.remove(id);
                    LobbyDeleted(IdResponse { id: id.to_string() }).to_json()
                }
                None => LobbyDetails(LobbyDetailsResponse {
                    lobby: lobby.clone(),
                })
                .to_json(),
            };
            broadcast_text_or_break(&response, broadcast_sender)
        }
        None => send_error_or_break(&format!("Lobby with id {} not found", &id), sender).await,
    }
}

async fn remove_player_from_lobby(player: String, lobby: &mut Lobby) -> Option<()> {
    let index = lobby.players.iter().position(|p| p == &player).unwrap();
    lobby.players.remove(index);
    match lobby.players.len() {
        0 => Some(()),
        _ => None,
    }
}

pub(crate) async fn list_games(sender: &mut Sender, state: Arc<WebSocketState>) -> ControlFlow {
    let game_hashmap = state.games.lock().await;
    let response = GameList(GameListResponse::from_game_hashmap(&game_hashmap)).to_json();
    send_text_or_break(&response, sender).await
}

pub(crate) async fn get_game_details(
    id: &str,
    player: &str,
    sender: &mut Sender,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    let game_hashmap = state.games.lock().await;
    match game_hashmap.get(id) {
        Some(game) => match game.players.iter().find(|&s| s.as_str() == player) {
            Some(_) => {
                send_text_or_break(
                    &GameDetails(game_to_json(id, game, player)).to_json(),
                    sender,
                )
                .await
            }
            None => {
                send_error_or_break(&format!("You don't belong to game with id {}", id), sender)
                    .await
            }
        },
        None => send_error_or_break(&format!("Game with id {} does not exist", id), sender).await,
    }
}

pub(crate) async fn game_move(
    payload: &GameMovePayload,
    player: String,
    sender: &mut Sender,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    match state.games.lock().await.get_mut(&payload.id) {
        Some(game) => {
            if !game.players.contains(&player) {
                return send_error_or_break(
                    &format!("You don't participate in game with id {}", payload.id),
                    sender,
                )
                .await;
            }

            match game.dispatch_payload(&payload.game_payload, &player) {
                Ok(_) => {
                    broadcast_game_to_players_or_break(&payload.id, &game, state.clone()).await
                }
                Err(game_error) => send_error_or_break(&game_error.to_string(), sender).await,
            }
        }
        None => {
            send_error_or_break(
                &format!("Game with id {} does not exist", payload.id),
                sender,
            )
            .await
        }
    }
}

pub(crate) async fn quit_game(
    id: &str,
    player: String,
    sender: &mut Sender,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    let mut games = state.games.lock().await;
    match games.get_mut(id) {
        Some(mut game) => {
            if !game.players.contains(&player) {
                return send_error_or_break(
                    &format!("You don't participate in game with id {}", id),
                    sender,
                )
                .await;
            }
            // forcefully finish game if the player quits during it
            match remove_player_from_game(player, &mut game).await {
                Some(_) => {
                    games.remove(id);
                    let response = GameDeleted(IdResponse { id: id.to_string() }).to_json();
                    broadcast_text_or_break(&response, broadcast_sender)
                }
                None => broadcast_game_to_players_or_break(id, &game, state.clone()).await,
            }
        }
        None => send_error_or_break(&format!("Game with id {} does not exist", id), sender).await,
    }
}

async fn remove_player_from_game(player: String, game: &mut Game) -> Option<()> {
    let index = game.players.iter().position(|p| p == &player).unwrap();
    game.players.remove(index);
    game.finished = true;

    match game.players.len() {
        0 => Some(()),
        _ => None,
    }
}

// TODO: maybe redis for shared state if scaling instances

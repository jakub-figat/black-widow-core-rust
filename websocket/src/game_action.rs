use crate::error::HandlerError::{ActionError, SenderError};
use crate::error::{HandlerError, HandlerResult};
use crate::lobby::Lobby;
use crate::network::{broadcast_game_to_players_or_break, broadcast_text, send_text};
use crate::payload::{
    CardExchangePayload, ClaimReadinessPayload, CreateLobbyPayload, InputCard, PlaceCardPayload,
};
use crate::response::{
    get_obfuscated_game_details_json, GameListResponse, IdResponse, ListedGame,
    LobbyDetailsResponse, LobbyListResponse, ToJson, WebSocketResponse::*,
};
use crate::WebSocketState;
use axum::extract::ws::Message;
use game::{Card, CardExchange, Game, GameSettings, RoundFinished, RoundInProgress};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

type Sender = mpsc::Sender<Message>;
type BroadcastSender = broadcast::Sender<Message>;

pub(crate) async fn list_lobbies(sender: &mut Sender, state: Arc<WebSocketState>) -> HandlerResult {
    let lobbies = state.lobbies.lock().await;
    let response = LobbyList(LobbyListResponse {
        lobbies: lobbies.clone(),
    });
    send_text(&response.to_json(), sender)
        .await
        .map_err(SenderError)
}

pub(crate) async fn get_lobby_details(
    id: &Uuid,
    sender: &mut Sender,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let lobbies = state.lobbies.lock().await;
    let lobby = lobbies
        .get(id)
        .ok_or(ActionError(format!("Lobby with id {} not found", &id)))?;

    let response = LobbyDetails(LobbyDetailsResponse {
        id: id.clone(),
        lobby: lobby.clone(),
    });

    send_text(&response.to_json(), sender)
        .await
        .map_err(SenderError)
}

pub(crate) async fn create_lobby(
    payload: &CreateLobbyPayload,
    player: &str,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let lobby = Lobby::new_by_player(payload.max_players, payload.max_score, player)
        .map_err(ActionError)?;
    let mut lobbies = state.lobbies.lock().await;
    let id = Uuid::new_v4();
    lobbies.insert(id.clone(), lobby.clone());

    let response = LobbyDetails(LobbyDetailsResponse {
        id,
        lobby: lobby.clone(),
    });
    broadcast_text(&response.to_json(), broadcast_sender).map_err(SenderError)
}

pub(crate) async fn join_lobby(
    id: &Uuid,
    player: &String,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let mut lobbies = state.lobbies.lock().await;
    let lobby = lobbies
        .get_mut(id)
        .ok_or(ActionError(format!("Lobby with id {} not found", &id)))?;

    if lobby.players.contains(player) {
        return Err(ActionError(format!(
            "You already belong to lobby with id {}",
            id
        )));
    }

    if let Some((game_id, game)) = add_player_to_lobby(lobby, player, state.clone()).await {
        lobbies.remove(id);

        broadcast_text(
            &LobbyDeleted(IdResponse { id: id.clone() }).to_json(),
            broadcast_sender,
        )
        .map_err(SenderError)?;

        broadcast_text(
            &ListedGame {
                id: game_id.clone(),
                players: game.players.clone(),
            }
            .to_json(),
            broadcast_sender,
        )
        .map_err(SenderError)?;

        return broadcast_game_to_players_or_break(&game_id, &game, state.clone())
            .await
            .map_err(SenderError);
    }

    let response = LobbyDetails(LobbyDetailsResponse {
        id: id.clone(),
        lobby: lobby.clone(),
    });
    broadcast_text(&response.to_json(), broadcast_sender).map_err(SenderError)
}

async fn add_player_to_lobby(
    lobby: &mut Lobby,
    player: &str,
    state: Arc<WebSocketState>,
) -> Option<(Uuid, Game)> {
    lobby.players.push(player.to_string());
    if lobby.players.len() == lobby.max_players {
        let game_id = Uuid::new_v4();
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
    id: &Uuid,
    player: &str,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let player = player.to_string();
    let mut lobbies = state.lobbies.lock().await;
    let mut lobby = lobbies
        .get_mut(id)
        .ok_or(ActionError(format!("Lobby with id {} not found", &id)))?;

    if !lobby.players.contains(&player) {
        return Err(ActionError(format!(
            "You don't belong to lobby with id {}",
            &id
        )));
    }

    let response = match remove_player_from_lobby(player, &mut lobby).await {
        Some(_) => {
            lobbies.remove(id);
            LobbyDeleted(IdResponse { id: id.clone() }).to_json()
        }
        None => LobbyDetails(LobbyDetailsResponse {
            id: id.clone(),
            lobby: lobby.clone(),
        })
        .to_json(),
    };

    broadcast_text(&response, broadcast_sender).map_err(SenderError)
}

async fn remove_player_from_lobby(player: String, lobby: &mut Lobby) -> Option<()> {
    let index = lobby.players.iter().position(|p| p == &player).unwrap();
    lobby.players.remove(index);
    match lobby.players.len() {
        0 => Some(()),
        _ => None,
    }
}

pub(crate) async fn list_games(sender: &mut Sender, state: Arc<WebSocketState>) -> HandlerResult {
    let game_hashmap = state.games.lock().await;
    let response = GameList(GameListResponse::from_game_hashmap(&game_hashmap)).to_json();
    send_text(&response, sender).await.map_err(SenderError)
}

pub(crate) async fn get_game_details(
    id: &Uuid,
    player: &String,
    sender: &mut Sender,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let games = state.games.lock().await;
    let game = games
        .get(id)
        .ok_or(ActionError(format!("Game with id {} does not exist", id)))?;

    check_player_in_game(id, &game, player)?;

    send_text(
        &get_obfuscated_game_details_json(id, &game, &player),
        sender,
    )
    .await
    .map_err(SenderError)
}

pub(crate) async fn card_exchange_move(
    payload: &CardExchangePayload,
    player: &String,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let mut games = state.games.lock().await;
    let game = games.get_mut(&payload.id).ok_or(ActionError(format!(
        "Game with id {} does not exist",
        &payload.id
    )))?;

    check_player_in_game(&payload.id, &game, player)?;
    check_game_finished(&game)?;

    match &mut game.state {
        CardExchange(step) => {
            let mut cards = HashSet::new();
            for card in &payload.cards_to_exchange {
                cards.insert(get_validated_card(card)?);
            }

            let game_payload = game::CardExchangePayload {
                cards_to_exchange: cards,
            };

            step.handle_payload(&game_payload, player)
                .map_err(|e| ActionError(e.to_string()))?;

            match step.should_switch() {
                true => {
                    game.state = RoundInProgress(step.clone().to_round_in_progress());
                    broadcast_game_to_players_or_break(&payload.id, &game, state.clone())
                        .await
                        .map_err(SenderError)
                }
                false => broadcast_game_to_players_or_break(&payload.id, &game, state.clone())
                    .await
                    .map_err(SenderError),
            }
        }
        _ => Err(ActionError(
            "Invalid game action, expected CardExchangeMove".to_string(),
        )),
    }
}

pub(crate) async fn place_card_move(
    payload: &PlaceCardPayload,
    player: &String,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let mut games = state.games.lock().await;
    let game = games.get_mut(&payload.id).ok_or(ActionError(format!(
        "Game with id {} does not exist",
        &payload.id
    )))?;

    check_player_in_game(&payload.id, &game, player)?;
    check_game_finished(&game)?;

    match &mut game.state {
        RoundInProgress(step) => {
            let game_payload = game::PlaceCardPayload {
                card: get_validated_card(&payload.card)?.clone(),
            };

            step.handle_payload(&game_payload, player)
                .map_err(|e| ActionError(e.to_string()))?;

            match step.should_switch() {
                true => {
                    game.state = RoundFinished(step.clone().to_round_finished());
                    broadcast_game_to_players_or_break(&payload.id, &game, state.clone())
                        .await
                        .map_err(SenderError)
                }
                false => broadcast_game_to_players_or_break(&payload.id, &game, state.clone())
                    .await
                    .map_err(SenderError),
            }
        }
        _ => Err(ActionError(
            "Invalid game action, expected RoundInProgress".to_string(),
        )),
    }
}

pub(crate) async fn claim_readiness_move(
    payload: &ClaimReadinessPayload,
    player: &String,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let mut games = state.games.lock().await;
    let game = games.get_mut(&payload.id).ok_or(ActionError(format!(
        "Game with id {} does not exist",
        &payload.id
    )))?;

    check_player_in_game(&payload.id, &game, player)?;
    check_game_finished(&game)?;

    match &mut game.state {
        RoundFinished(step) => {
            let game_payload = game::ClaimReadinessPayload {
                ready: payload.ready,
            };

            step.handle_payload(&game_payload, player);

            match step.should_switch() {
                true => {
                    game.state = CardExchange(step.clone().to_card_exchange());
                    broadcast_game_to_players_or_break(&payload.id, &game, state.clone())
                        .await
                        .map_err(SenderError)
                }
                false => broadcast_game_to_players_or_break(&payload.id, &game, state.clone())
                    .await
                    .map_err(SenderError),
            }
        }
        _ => Err(ActionError(
            "Invalid game action, expected RoundFinished".to_string(),
        )),
    }
}

pub(crate) async fn quit_game(
    id: &Uuid,
    player: String,
    broadcast_sender: &mut BroadcastSender,
    state: Arc<WebSocketState>,
) -> HandlerResult {
    let mut games = state.games.lock().await;
    let mut game = games
        .get_mut(id)
        .ok_or(ActionError(format!("Game with id {} does not exist", id)))?;

    if !game.players.contains(&player) {
        return Err(ActionError(format!(
            "You don't participate in game with id {}",
            id
        )));
    }

    // forcefully finish game if the player quits during it
    match remove_player_from_game(player, &mut game).await {
        Some(_) => {
            games.remove(id);

            let response = GameDeleted(IdResponse { id: id.clone() }).to_json();
            broadcast_text(&response, broadcast_sender).map_err(SenderError)
        }
        None => broadcast_game_to_players_or_break(id, &game, state.clone())
            .await
            .map_err(SenderError),
    }
}

async fn remove_player_from_game(player: String, game: &mut Game) -> Option<()> {
    let index = game.players.iter().position(|p| p == &player).unwrap();
    game.players.remove(index);
    game.is_finished = true;

    match game.players.len() {
        0 => Some(()),
        _ => None,
    }
}

fn check_player_in_game(id: &Uuid, game: &Game, player: &String) -> HandlerResult {
    if !game.players.contains(player) {
        return Err(ActionError(format!(
            "You don't participate in game with id {}",
            id
        )));
    }
    Ok(())
}

fn check_game_finished(game: &Game) -> HandlerResult {
    if game.is_finished {
        return Err(ActionError("Game is already finished".to_string()));
    }
    Ok(())
}

fn get_validated_card(card: &InputCard) -> Result<Card, HandlerError> {
    Card::new(card.suit, card.value).map_err(ActionError)
}

// TODO: maybe redis for shared state if scaling instances

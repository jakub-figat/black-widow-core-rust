use crate::network::{broadcast_game_to_players, broadcast_text, BroadcastSender};
use crate::response::WebSocketResponse::LobbyDeleted;
use crate::response::{IdResponse, ToJson};
use crate::WebSocketState;
use game::helper::{check_if_player_has_only_one_suit_remaining, check_if_player_has_suit};
use game::CardSuit::Heart;
use game::{Card, CardExchange, CardSuit, Game, RoundFinished, RoundInProgress};
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

static LOBBY_TIMEOUT_MINUTES: u64 = 20;
static GAME_MOVE_TIMEOUT_SECONDS: u64 = 90;

pub(crate) async fn schedule_delete_lobby(
    id: Uuid,
    mut broadcast_sender: BroadcastSender,
    state: Arc<WebSocketState>,
) {
    sleep(Duration::from_secs(LOBBY_TIMEOUT_MINUTES * 60)).await;

    let mut lobbies = state.lobbies.lock().await;
    match lobbies.remove(&id) {
        Some(_) => {
            tracing::info!("Lobby with id {} timed out", &id);
            let response = LobbyDeleted(IdResponse { id: id.clone() }).to_json();
            if let Err(error) = broadcast_text(&response, &mut broadcast_sender) {
                tracing::error!(error);
            }
        }
        None => tracing::error!("Failed to delete lobby with id {} after timeout", id),
    }
}

pub(crate) async fn schedule_random_game_move(
    id: Uuid,
    player: &String,
    state: Arc<WebSocketState>,
) {
    sleep(Duration::from_secs(GAME_MOVE_TIMEOUT_SECONDS)).await;

    let mut games = state.games.lock().await;
    match games.get_mut(&id) {
        Some(game) => {
            tracing::info!("Player {} move timed out in game with id {}", player, &id);
            match &mut game.state {
                CardExchange(step) => {
                    let cards = get_random_cards_for_exchange(&step.player_decks[player]);
                    let game_payload = game::CardExchangePayload {
                        cards_to_exchange: cards,
                    };

                    step.handle_payload(&game_payload, player).unwrap();
                    if step.should_switch() {
                        game.state = RoundInProgress(step.clone().to_round_in_progress());
                    }
                    broadcast_game_to_players_or_trace_error(&id, &game, state.clone()).await;
                }
                RoundInProgress(step) => {
                    let card =
                        get_random_card_to_place(&step.player_decks[player], step.state.table_suit);
                    let game_payload = game::PlaceCardPayload { card };

                    step.handle_payload(&game_payload, player).unwrap();

                    if step.should_switch() {
                        game.state = RoundFinished(step.clone().to_round_finished());
                    }
                    broadcast_game_to_players_or_trace_error(&id, &game, state.clone()).await;
                }
                RoundFinished(step) => {
                    let game_payload = game::ClaimReadinessPayload { ready: true };

                    step.handle_payload(&game_payload, player);

                    if step.should_switch() {
                        game.state = CardExchange(step.clone().to_card_exchange());
                    }
                    broadcast_game_to_players_or_trace_error(&id, &game, state.clone()).await;
                }
            }
        }
        None => tracing::error!(
            "Game with id {} does not exist for player timeout {}",
            &id,
            &player
        ),
    }
}

async fn broadcast_game_to_players_or_trace_error(
    id: &Uuid,
    game: &Game,
    state: Arc<WebSocketState>,
) {
    if let Err(error) = broadcast_game_to_players(id, game, state.clone()).await {
        tracing::error!(error);
    }
}

fn get_random_cards_for_exchange(cards: &HashSet<Card>) -> HashSet<Card> {
    let mut rng = thread_rng();
    HashSet::from_iter(
        cards
            .iter()
            .choose_multiple(&mut rng, 3)
            .into_iter()
            .cloned(),
    )
}

fn get_random_card_to_place(cards: &HashSet<Card>, table_suit: Option<CardSuit>) -> Card {
    let cards_iter = cards.iter();

    let suitable_cards: HashSet<&Card> = match table_suit {
        Some(suit) if check_if_player_has_suit(cards, suit) => {
            cards_iter.filter(|&card| card.suit == suit).collect()
        }
        None if !check_if_player_has_only_one_suit_remaining(cards, Heart) => {
            cards_iter.filter(|&card| card.suit != Heart).collect()
        }
        _ => cards_iter.collect(),
    };

    let mut rng = thread_rng();
    suitable_cards
        .into_iter()
        .choose(&mut rng)
        .cloned()
        .unwrap()
}

pub(crate) fn cancel_lobby_timeout(id: &Uuid, state: Arc<WebSocketState>) {
    let mut lobby_timeouts = state.lobby_timeouts.lock().unwrap();
    match lobby_timeouts.remove(&id) {
        Some(timeout_handle) => timeout_handle.abort(),
        None => tracing::error!("Lobby timeout with id {} not found", &id),
    }
}

pub(crate) fn cancel_player_timeout(id: &Uuid, player: &String, state: Arc<WebSocketState>) {
    let mut game_timeouts = state.game_timeouts.lock().unwrap();
    match game_timeouts.get_mut(&id) {
        Some(player_timeouts) => match player_timeouts.remove(player) {
            Some(player_timeout) => player_timeout.abort(),
            None => tracing::error!("Player {} timeout not found", player),
        },
        None => tracing::error!("Game timeout with id {} not found", id),
    }
}

use crate::response::WebSocketResponse::Error;
use crate::response::{get_obfuscated_game_details_json, ErrorResponse, ToJson};
use crate::WebSocketState;
use axum::extract::ws::Message;
use game::Game;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

pub(crate) async fn send_text(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> Result<(), String> {
    sender
        .send(Message::Text(text.to_string()))
        .await
        .map_err(|e| e.to_string())
}

pub(crate) fn broadcast_text(
    text: &str,
    broadcast_sender: &mut broadcast::Sender<Message>,
) -> Result<(), String> {
    broadcast_sender
        .send(Message::Text(text.to_string()))
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub(crate) async fn broadcast_game_to_players_or_break(
    id: &Uuid,
    game: &Game,
    state: Arc<WebSocketState>,
) -> Result<(), String> {
    let player_connections = state.player_connections.read().await;
    for player in &game.players {
        match player_connections.get(player).cloned() {
            Some(mut sender) => {
                send_text(
                    &get_obfuscated_game_details_json(id, game, player),
                    &mut sender,
                )
                .await?
            }
            None => tracing::warn!(
                "Tried to send game with id {} to disconnected player {}",
                id,
                player
            ),
        }
    }
    Ok(())
}

pub(crate) async fn send_error(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> Result<(), String> {
    send_text(
        &Error(ErrorResponse {
            detail: text.to_string(),
        })
        .to_json(),
        sender,
    )
    .await
}

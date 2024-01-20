use crate::response::ErrorResponse;
use axum::extract::ws::Message;
use game::Card;
use std::collections::{HashMap, HashSet};
use std::ops;
use std::str::FromStr;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;
use crate::error::ValidationError;
use crate::payload::IdPayload;

type ControlFlow = ops::ControlFlow<(), ()>;

pub(crate) async fn send_text_or_break(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow {
    if sender.send(Message::Text(text.to_string())).await.is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub(crate) fn broadcast_text_or_break(
    text: &str,
    sender: &mut broadcast::Sender<Message>,
) -> ControlFlow {
    if sender.send(Message::Text(text.to_string())).is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub(crate) async fn send_error_or_break(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow {
    send_text_or_break(&ErrorResponse::json_from_detail(text), sender).await
}

pub(crate) fn get_obfuscated_player_cards(
    decks: &HashMap<String, HashSet<Card>>,
    player: &str,
) -> HashMap<String, usize> {
    decks
        .iter()
        .filter(|(k, _)| k.as_str() != player)
        .map(|(k, v)| (k.clone(), v.len()))
        .collect()
}

pub(crate) fn get_obfuscated_exchange_cards(
    cards_to_exchange: &HashMap<String, HashSet<Card>>,
    player: &str,
) -> HashMap<String, bool> {
    cards_to_exchange
        .iter()
        .filter(|(k, _)| k.as_str() != player)
        .map(|(k, v)| (k.clone(), !v.is_empty()))
        .collect()
}

pub(crate) fn parse_uuid_from_payload(s: &str) -> Result<String, ValidationError> {
    match serde_json::from_str::<IdPayload>(&s) {
        Ok(payload) => {
            match Uuid::from_str(&payload.id) {
                Ok(parsed_uuid) => Ok(parsed_uuid.to_string()),
                Err(error) => Err(ValidationError(error.to_string()))
            }
        }
        Err(error) => Err(ValidationError(error.to_string()))
    }
}

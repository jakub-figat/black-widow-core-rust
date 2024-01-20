use crate::response::ErrorResponse;
use axum::extract::ws::Message;
use game::Card;
use std::collections::{HashMap, HashSet};
use std::ops;
use std::str::FromStr;
use tokio::sync::{broadcast, mpsc};
use uuid::{Error, Uuid};

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

pub(crate) fn parse_uuid(s: &str) -> Result<String, Error> {
    let parsed_uuid = Uuid::from_str(&s)?;
    Ok(parsed_uuid.to_string())
}

use crate::error::ValidationError;
use crate::payload::IdPayload;
use game::Card;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use uuid::Uuid;

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

pub(crate) fn parse_uuid(id: &str) -> Result<String, ValidationError> {
    match Uuid::from_str(&id) {
        Ok(parsed_uuid) => Ok(parsed_uuid.to_string()),
        Err(error) => Err(ValidationError(error.to_string())),
    }
}

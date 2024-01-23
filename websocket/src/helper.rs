use game::Card;
use std::collections::{HashMap, HashSet};

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

use std::collections::HashMap;
use crate::card::Card;

pub fn get_players_cards_hashmap(players: &[String]) -> HashMap<String, Vec<Card>> {
    let mut hashmap = HashMap::new();
    for player in players {
        hashmap.entry(player.clone()).or_insert(vec![]);
    }

    hashmap
}
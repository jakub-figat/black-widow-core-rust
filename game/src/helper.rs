use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::card::Card;
use crate::card::CardSuit::Club;
use crate::error::GameError;

pub fn pick_player_with_starting_card(
    player_decks: &HashMap<String, HashSet<Card>>
) -> Option<(String, Card)> {
    let starting_card = match player_decks.len() {
        3 => Card::new(Club, 3),
        4 => Card::new(Club, 2),
        _ => panic!("Invalid number of players")
    };

    for (player, deck) in player_decks {
        if deck.contains(&starting_card) {
            return Some((player.to_string(), starting_card));
        }
    }

    None
}

pub fn get_player_to_player_map(players: &[String]) -> HashMap<String, String> {
    HashMap::from_iter(
        players.iter()
            .enumerate()
            .map(|(i, player)| (player.clone(), players[(i+1) % players.len()].clone()))
            .collect::<HashMap<_, _>>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_player_with_starting_card_from_3_decks() {
        let player_decks = HashMap::from(
            [
                ("1".to_string(), HashSet::from([Card::new(Club, 10)])),
                ("2".to_string(), HashSet::from([Card::new(Club, 11)])),
                ("3".to_string(), HashSet::from([Card::new(Club, 3)]))
            ],
        );

        assert_eq!(
            Some(("3".to_string(), Card::new(Club, 3))),
            pick_player_with_starting_card(&player_decks)
        );
    }

    #[test]
    fn pick_player_with_starting_card_from_4_decks() {
        let player_decks = HashMap::from(
            [
                ("1".to_string(), HashSet::from([Card::new(Club, 10)])),
                ("2".to_string(), HashSet::from([Card::new(Club, 11)])),
                ("3".to_string(), HashSet::from([Card::new(Club, 2)])),
                ("4".to_string(), HashSet::from([Card::new(Club, 3)]))
            ],
        );

        assert_eq!(
            Some(("3".to_string(), Card::new(Club, 2))),
            pick_player_with_starting_card(&player_decks)
        );
    }

    #[test]
    fn pick_player_with_starting_card_when_there_is_no_starting_card() {
        let player_decks = HashMap::from(
            [
                ("1".to_string(), HashSet::from([Card::new(Club, 10)])),
                ("2".to_string(), HashSet::from([Card::new(Club, 11)])),
                ("3".to_string(), HashSet::from([Card::new(Club, 4)])),
            ],
        );

        assert_eq!(None, pick_player_with_starting_card(&player_decks));
    }

    #[test]
    #[should_panic(expected = "Invalid number of players")]
    fn pick_player_with_starting_card_should_panic_with_invalid_number_of_players() {
        let player_decks = HashMap::from(
            [
                ("1".to_string(), HashSet::from([Card::new(Club, 10)])),
            ]
        );
        pick_player_with_starting_card(&player_decks);
    }

    #[test]
    fn get_player_to_player_map_for_3_players() {
        let players = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ];
        let expected_map = HashMap::from(
            [
                ("1".to_string(), "2".to_string()),
                ("2".to_string(), "3".to_string()),
                ("3".to_string(), "1".to_string()),
            ]
        );

        assert_eq!(get_player_to_player_map(&players), expected_map)
    }

    #[test]
    fn get_player_to_player_map_for_4_players() {
        let players = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ];
        let expected_map = HashMap::from(
            [
                ("1".to_string(), "2".to_string()),
                ("2".to_string(), "3".to_string()),
                ("3".to_string(), "4".to_string()),
                ("4".to_string(), "1".to_string()),
            ]
        );

        assert_eq!(get_player_to_player_map(&players), expected_map)
    }
}
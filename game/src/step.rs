use std::collections::{HashMap, HashSet};
use crate::card::{Card, CardSuit};
use crate::error::{GameError, GameResult};

pub mod card_exchange;
pub mod round_in_progress;
pub mod round_finished;


#[derive(Debug, Clone)]
pub struct GameStep<T> {
    players: Vec<String>,
    player_to_player_map: HashMap<String, String>,
    scores: HashMap<String, usize>,
    player_decks: HashMap<String, HashSet<Card>>,
    state: T
}

impl<T> GameStep<T> {
    fn validate_player_has_card(&self, card: &Card, player: &str) -> GameResult<()> {
        if !&self.player_decks.get(player).unwrap().contains(card) {
            Err(
                GameError::InvalidAction(
                    format!("Player {} does not have a card {}", player, card)
                )
            )?
        }

        Ok(())
    }

    fn check_if_player_has_only_one_suit_remaining(&self, player: &str, suit: CardSuit) -> bool {
        self.player_decks.get(player).unwrap().iter()
            .all(|card| card.suit == suit)
    }

    fn check_if_player_has_suit(&self, player: &str, suit: CardSuit) -> bool {
        self.player_decks.get(player).unwrap().iter()
            .any(|card| card.suit == suit)

    }
}


#[cfg(test)]
mod tests {
    use crate::card::CardSuit::Spade;
    use super::*;

    fn get_players() -> Vec<String> {
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ]
    }

    #[test]
    fn player_does_not_have_a_card_when_player_does_not_have_a_card() {
        let players = get_players();
        let step = GameStep::empty_from_players(&players);
        let card = Card::new(Spade, 2);

        assert_eq!(
            step.validate_player_has_card(&card, &players[0]),
            Err(
                GameError::InvalidAction("Player 1 does not have a card SPADE_2".to_string())
            )
        );
    }

    #[test]
    fn player_does_not_have_a_card_when_player_has_a_card() {
        let players = get_players();
        let mut step = GameStep::empty_from_players(&players);
        let card = Card::new(Spade, 2);

        step.player_decks.get_mut(&players[0]).unwrap().insert(card.clone());

        assert_eq!(
            step.validate_player_has_card(&card, &players[0]),
            Ok(())
        );
    }
}
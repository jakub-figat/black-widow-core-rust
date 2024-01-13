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
    use crate::card::CardSuit::{Club, Spade};
    use crate::payload::{CardExchangePayload, PlaceCardPayload};
    use crate::r#trait::PayloadHandler;
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

    #[test]
    fn play_all_steps() {
        let players = get_players();
        let mut exchange_step = GameStep::empty_from_players(&players);
        let initial_decks = HashMap::from(
            [
                (
                    "1".to_string(),
                    HashSet::from(
                        [
                            Card::new(Club, 6),
                            Card::new(Spade, 7),
                            Card::new(Spade, 8)
                        ]
                    )
                ),
                (
                    "2".to_string(),
                    HashSet::from(
                        [
                            Card::new(Club, 9),
                            Card::new(Spade, 10),
                            Card::new(Spade, 12)
                        ]
                    )
                ),
                (
                    "3".to_string(),
                    HashSet::from(
                        [
                            Card::new(Club, 3),
                            Card::new(Spade, 4),
                            Card::new(Spade, 5)
                        ]
                    )
                )
            ]
        );

        exchange_step.player_decks = initial_decks.clone();

        // card exchange
        for (player, cards) in initial_decks {
            let payload = CardExchangePayload {cards_to_exchange: cards};
            exchange_step.validate_payload(&payload, &player).unwrap();
            exchange_step.dispatch_payload(&payload, &player);
        }

        assert!(exchange_step.should_switch());

        // round in progress, two rounds
        // current_decks: [
        //     1: [club_3, spade_4, spade_5],
        //     2: [club_6, spade_7, spade_8],
        //     3: [club_9, spade_10, spade_12],
        // ]
        let mut round_in_progress_step = exchange_step.to_round_in_progress();

        let payload = PlaceCardPayload {card: Card::new(Club, 6)};
        round_in_progress_step.validate_payload(&payload, "2").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "2");

        let payload = PlaceCardPayload {card: Card::new(Club, 9)};
        round_in_progress_step.validate_payload(&payload, "3").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "3");
        assert_eq!(round_in_progress_step.state.current_player, "3".to_string());


        // 2 round
        // current_decks: [
        //     1: [spade_4, spade_5],
        //     2: [spade_7, spade_8],
        //     3: [spade_10, spade_12], <- turn
        // ]
        let payload = PlaceCardPayload {card: Card::new(Spade, 12)};
        round_in_progress_step.validate_payload(&payload, "3").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "3");

        let payload = PlaceCardPayload {card: Card::new(Spade, 4)};
        round_in_progress_step.validate_payload(&payload, "1").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "1");

        let payload = PlaceCardPayload {card: Card::new(Spade, 7)};
        round_in_progress_step.validate_payload(&payload, "2").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "2");

        assert_eq!(round_in_progress_step.state.current_player, "3".to_string());
        assert_eq!(round_in_progress_step.scores["3"], 13);


        // 3 round
        // current_decks: [
        //     1: [spade_5],
        //     2: [spade_8],
        //     3: [spade_10], <- turn
        // ]
        let payload = PlaceCardPayload {card: Card::new(Spade, 10)};
        round_in_progress_step.validate_payload(&payload, "3").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "3");

        let payload = PlaceCardPayload {card: Card::new(Spade, 5)};
        round_in_progress_step.validate_payload(&payload, "1").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "1");

        let payload = PlaceCardPayload {card: Card::new(Spade, 8)};
        round_in_progress_step.validate_payload(&payload, "2").unwrap();
        round_in_progress_step.dispatch_payload(&payload, "2");
        assert!(round_in_progress_step.should_switch());


        // game finished
        let round_finished_step = round_in_progress_step.to_round_finished();
        assert!(!round_finished_step.should_switch());
        assert!(round_finished_step.game_finished(10));
    }
}
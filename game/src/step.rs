use crate::card::{Card, CardSuit};
use crate::error::{GameError, GameResult};
use std::collections::{HashMap, HashSet};

pub mod card_exchange;
pub mod round_finished;
pub mod round_in_progress;

#[derive(Debug, Clone)]
pub struct GameStep<S> {
    pub players: Vec<String>,
    pub player_to_player_map: HashMap<String, String>,
    pub scores: HashMap<String, usize>,
    pub player_decks: HashMap<String, HashSet<Card>>,
    pub state: S,
}

impl<S> GameStep<S> {
    fn validate_player_has_card(&self, card: &Card, player: &str) -> GameResult<()> {
        if !&self.player_decks.get(player).unwrap().contains(card) {
            Err(GameError(format!(
                "Player {} does not have a card {}",
                player, card
            )))?
        }

        Ok(())
    }

    fn check_if_player_has_only_one_suit_remaining(&self, player: &str, suit: CardSuit) -> bool {
        self.player_decks
            .get(player)
            .unwrap()
            .iter()
            .all(|card| card.suit == suit)
    }

    fn check_if_player_has_suit(&self, player: &str, suit: CardSuit) -> bool {
        self.player_decks
            .get(player)
            .unwrap()
            .iter()
            .any(|card| card.suit == suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardSuit::{Club, Spade};
    use crate::payload::{CardExchangePayload, PlaceCardPayload};

    fn get_players() -> Vec<String> {
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    }

    #[test]
    fn player_does_not_have_a_card_when_player_does_not_have_a_card() {
        let players = get_players();
        let step = GameStep::empty_from_players(&players);
        let card = Card::new(Spade, 2).unwrap();

        assert_eq!(
            step.validate_player_has_card(&card, &players[0]),
            Err(GameError(
                "Player 1 does not have a card SPADE_2".to_string()
            ))
        );
    }

    #[test]
    fn player_does_not_have_a_card_when_player_has_a_card() {
        let players = get_players();
        let mut step = GameStep::empty_from_players(&players);
        let card = Card::new(Spade, 2).unwrap();

        step.player_decks
            .get_mut(&players[0])
            .unwrap()
            .insert(card.clone());

        assert_eq!(step.validate_player_has_card(&card, &players[0]), Ok(()));
    }

    #[test]
    fn play_all_steps() {
        let players = get_players();
        let mut exchange_step = GameStep::empty_from_players(&players);
        let initial_decks = HashMap::from([
            (
                "1".to_string(),
                HashSet::from([
                    Card::new(Club, 6).unwrap(),
                    Card::new(Spade, 7).unwrap(),
                    Card::new(Spade, 8).unwrap(),
                ]),
            ),
            (
                "2".to_string(),
                HashSet::from([
                    Card::new(Club, 9).unwrap(),
                    Card::new(Spade, 10).unwrap(),
                    Card::new(Spade, 12).unwrap(),
                ]),
            ),
            (
                "3".to_string(),
                HashSet::from([
                    Card::new(Club, 3).unwrap(),
                    Card::new(Spade, 4).unwrap(),
                    Card::new(Spade, 5).unwrap(),
                ]),
            ),
        ]);

        exchange_step.player_decks = initial_decks.clone();

        // card exchange
        for (player, cards) in initial_decks {
            let payload = CardExchangePayload {
                cards_to_exchange: cards,
            };
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

        let payload = PlaceCardPayload {
            card: Card::new(Club, 6).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "2")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "2");

        let payload = PlaceCardPayload {
            card: Card::new(Club, 9).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "3")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "3");
        assert_eq!(round_in_progress_step.state.current_player, "3".to_string());

        // 2 round
        // current_decks: [
        //     1: [spade_4, spade_5],
        //     2: [spade_7, spade_8],
        //     3: [spade_10, spade_12], <- turn
        // ]
        let payload = PlaceCardPayload {
            card: Card::new(Spade, 12).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "3")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "3");

        let payload = PlaceCardPayload {
            card: Card::new(Spade, 4).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "1")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "1");

        let payload = PlaceCardPayload {
            card: Card::new(Spade, 7).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "2")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "2");

        assert_eq!(round_in_progress_step.state.current_player, "3".to_string());
        assert_eq!(round_in_progress_step.scores["3"], 13);

        // 3 round
        // current_decks: [
        //     1: [spade_5],
        //     2: [spade_8],
        //     3: [spade_10], <- turn
        // ]
        let payload = PlaceCardPayload {
            card: Card::new(Spade, 10).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "3")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "3");

        let payload = PlaceCardPayload {
            card: Card::new(Spade, 5).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "1")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "1");

        let payload = PlaceCardPayload {
            card: Card::new(Spade, 8).unwrap(),
        };
        round_in_progress_step
            .validate_payload(&payload, "2")
            .unwrap();
        round_in_progress_step.dispatch_payload(&payload, "2");
        assert!(round_in_progress_step.should_switch());

        // game is_finished
        let round_finished_step = round_in_progress_step.to_round_finished();
        assert!(!round_finished_step.should_switch());
        assert!(round_finished_step.game_finished(10));
    }
}

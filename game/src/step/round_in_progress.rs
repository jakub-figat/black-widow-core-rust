use std::collections::{HashMap, HashSet};
use crate::card::{Card, CardSuit};
use crate::card::CardSuit::Heart;
use crate::error::{GameError, GameResult};
use crate::payload::PlaceCardPayload;
use crate::r#trait::PayloadHandler;
use crate::step::GameStep;
use crate::step::round_finished::RoundFinishedState;

impl GameStep<RoundInProgressState> {
    fn validate_current_player(&self, player: &str) -> GameResult<()> {
        if self.state.current_player != player {
            Err(
                GameError::InvalidAction(
                    format!("Cannot make move, current player is {}", self.state.current_player)
                )
            )?
        }

        Ok(())
    }

    fn validate_placed_suit(&self, placed_suit: CardSuit, table_suit: CardSuit, player: &str) -> GameResult<()> {
        if placed_suit != table_suit && self.check_if_player_has_suit(player, table_suit) {
            Err(
                GameError::InvalidAction(
                    format!(
                        "Player {} tried to place {}, despite having {} in deck",
                        &player,
                        placed_suit,
                        table_suit
                    )
                )
            )?
        }

        Ok(())
    }

    fn validate_only_heart_left(&self, player: &str) -> GameResult<()> {
        if !self.check_if_player_has_only_one_suit_remaining(player, Heart) {
            Err(
                GameError::InvalidAction(
                    format!(
                        "Player {} tried to place Heart suit on the table, despite having other suits left",
                        player
                    )
                )
            )?
        }

        Ok(())
    }

    pub fn place_card(&mut self, card: &Card) {
        let current_player = &self.state.current_player;
        self.player_decks.get_mut(current_player).unwrap().remove(card);
        self.state.cards_on_table.insert(current_player.clone(), card.clone());
    }

    fn get_scoring_player(&self) -> String {
        self.state.cards_on_table.iter()
            .filter(|(_, card)| card.suit == self.state.table_suit.unwrap())
            .max_by_key(|(_ ,card)| card.value)
            .unwrap()
            .0
            .clone()
    }

    fn get_total_score_of_cards_on_table(&self) -> usize {
        self.state.cards_on_table.iter()
            .map(|(_, card)| card.score)
            .sum()
    }

    fn prepare_table_for_next_turn(&mut self) {
        let scoring_player = self.get_scoring_player();
        let score = self.get_total_score_of_cards_on_table();

        *self.scores.entry(scoring_player.clone()).or_insert(0) += score;
        self.state.current_player = scoring_player;

        self.state.cards_on_table = HashMap::new();
        self.state.table_suit = None;
    }

    pub fn should_switch(&self) -> bool {
        self.player_decks.iter()
            .all(|(_, cards)| cards.is_empty())
    }

    pub fn to_round_finished(self) -> GameStep<RoundFinishedState> {
        GameStep {
            players: self.players,
            player_to_player_map: self.player_to_player_map,
            scores: self.scores,
            player_decks: self.player_decks,
            state: RoundFinishedState { players_ready: HashMap::new()}
        }
    }
}

impl PayloadHandler<'_, PlaceCardPayload> for GameStep<RoundInProgressState> {
    fn validate_payload(&self, payload: &PlaceCardPayload, player: &str) -> GameResult<()> {
        self.validate_current_player(player)?;
        self.validate_player_has_card(&payload.card, &self.state.current_player)?;

        match self.state.table_suit {
            Some(table_suit) => {
                self.validate_placed_suit(payload.card.suit, table_suit, player)
            }
            None => {
                if payload.card.suit == Heart {
                    self.validate_only_heart_left(player)?
                }

                Ok(())
            }
        }
    }

    fn dispatch_payload(&mut self, payload: &PlaceCardPayload,  player: &str) {
        self.place_card(&payload.card);

        if self.state.table_suit.is_none() {
            self.state.table_suit = Some(payload.card.suit);
        }

        if self.state.cards_on_table.len() == self.players.len() {
            self.prepare_table_for_next_turn();
        } else {
            self.state.current_player = self.player_to_player_map[player].clone();
        }
    }
}


#[derive(Debug, Clone)]
pub struct RoundInProgressState {
    pub current_player: String,
    pub table_suit: Option<CardSuit>,
    pub cards_on_table: HashMap<String, Card>
}

#[cfg(test)]
mod tests {
    use crate::card::CardSuit::{Diamond, Spade};
    use crate::helper::get_player_to_player_map;
    use super::*;

    fn get_players() -> Vec<String> {
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ]
    }

    fn get_step_from_players(players: &Vec<String>) -> GameStep<RoundInProgressState> {
        GameStep {
            players: players.clone(),
            player_to_player_map: get_player_to_player_map(&players),
            scores: HashMap::new(),
            player_decks: HashMap::from_iter(
                players.iter()
                    .cloned()
                    .map(|player| (player, HashSet::new()))
                    .collect::<HashMap<_, _>>()
            ),
            state: RoundInProgressState {
                current_player: players[0].clone(),
                table_suit: None,
                cards_on_table: HashMap::new()
            }
        }
    }

    #[test]
    fn validate_current_player_when_current_player() {
        let players = get_players();
        let step = get_step_from_players(&players);

        assert!(step.validate_current_player("1").is_ok());
    }

    #[test]
    fn validate_current_player_when_not_current_player() {
        let players = get_players();
        let step = get_step_from_players(&players);
        let expected_error = Err(
            GameError::InvalidAction(
                "Cannot make move, current player is 1".to_string()
            )
        );

        assert_eq!(step.validate_current_player("2"), expected_error);
    }

    #[test]
    fn validate_payload_when_player_places_matching_suit() {
        let players = get_players();
        let mut step = get_step_from_players(&players);
        step.state.table_suit = Some(Spade);

        let card = Card::new(Spade, 2);
        step.player_decks.get_mut(&players[0]).unwrap().insert(card.clone());
        let payload = PlaceCardPayload {card};

        assert!(step.validate_payload(&payload, &players[0]).is_ok());
    }

    #[test]
    fn validate_payload_when_places_places_mismatched_suit_despite_having_matching_suit() {
        let players = get_players();
        let mut step = get_step_from_players(&players);
        step.state.table_suit = Some(Spade);

        let card = Card::new(Diamond, 2);
        let player_deck = step.player_decks.get_mut(&players[0]).unwrap();
        player_deck.insert(card.clone());
        player_deck.insert(Card::new(Spade, 2));

        let payload = PlaceCardPayload {card};

        let expected_error = Err(
            GameError::InvalidAction(
                "Player 1 tried to place DIAMOND, despite having SPADE in deck".to_string()
            )
        );
        assert_eq!(step.validate_payload(&payload, &players[0]), expected_error);
    }

    #[test]
    fn validate_payload_when_places_places_mismatched_suit_and_does_not_have_matching_suit() {
        let players = get_players();
        let mut step = get_step_from_players(&players);
        step.state.table_suit = Some(Spade);

        let card = Card::new(Diamond, 2);
        step.player_decks.get_mut(&players[0]).unwrap().insert(card.clone());

        let payload = PlaceCardPayload {card};
        assert!(step.validate_payload(&payload, &players[0]).is_ok());
    }

    #[test]
    fn validate_payload_when_table_suit_is_none_and_player_places_heart_despite_having_other_suits() {
        let players = get_players();
        let mut step = get_step_from_players(&players);
        step.player_decks.get_mut(&players[0]).unwrap().insert(Card::new(Spade, 2));
        let card = Card::new(Heart, 2);

        step.player_decks.get_mut(&players[0]).unwrap().insert(card.clone());
        let payload = PlaceCardPayload {card};

        let expected_error = Err(
            GameError::InvalidAction(
                "Player 1 tried to place Heart suit on the table, despite having other suits left".to_string()
            )
        );
        assert_eq!(step.validate_payload(&payload, &players[0]), expected_error);
    }

    #[test]
    fn validate_payload_when_table_suit_is_none_and_heart_is_placed_and_its_player_only_card() {
        let players = get_players();
        let mut step = get_step_from_players(&players);

        let card = Card::new(Heart, 2);
        step.player_decks.get_mut(&players[0]).unwrap().insert(card.clone());
        let payload = PlaceCardPayload {card};

        assert!(step.validate_payload(&payload, &players[0]).is_ok());
    }

    #[test]
    fn validate_payload_when_table_suit_is_none_and_non_heart_is_placed() {
        let players = get_players();
        let mut step = get_step_from_players(&players);

        let card = Card::new(Spade, 2);
        step.player_decks.get_mut(&players[0]).unwrap().insert(card.clone());
        let payload = PlaceCardPayload {card};

        assert!(step.validate_payload(&payload, &players[0]).is_ok());
    }

    #[test]
    fn dispatch_payload_when_decks_are_left_empty() {
        let players = get_players();
        let mut step = get_step_from_players(&players);
        let (card_1, card_2, card_3) = (
            Card::new(Spade, 5), Card::new(Spade, 12), Card::new(Spade, 4)
        );
        step.player_decks = HashMap::from(
            [
                ("1".to_string(), HashSet::from([card_1.clone()])),
                ("2".to_string(), HashSet::from([card_2.clone()])),
                ("3".to_string(), HashSet::from([card_3.clone()])),
            ]
        );

        step.dispatch_payload(&PlaceCardPayload {card: card_1}, &players[0]);
        step.dispatch_payload(&PlaceCardPayload {card: card_2}, &players[1]);
        step.dispatch_payload(&PlaceCardPayload {card: card_3}, &players[2]);

        assert_eq!(&step.state.current_player, "2");
        assert_eq!(step.scores, HashMap::from([("2".to_string(), 13)]));
        assert!(step.state.cards_on_table.is_empty());
        assert!(step.state.table_suit.is_none());
        assert!(step.should_switch());
    }
}

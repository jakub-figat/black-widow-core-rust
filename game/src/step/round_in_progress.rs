use std::collections::HashMap;
use crate::card::{Card, CardSuit};
use crate::card::CardSuit::Heart;
use crate::error::{GameError, GameResult};
use crate::payload::PlaceCardPayload;
use crate::step::GameStep;

impl GameStep<RoundInProgressState> {
    pub fn validate_payload(&self, payload: &PlaceCardPayload) -> GameResult<()> {
        let player = &self.state.current_player;
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

    pub fn dispatch_payload(&mut self, payload: &PlaceCardPayload) {
        self.place_card(&payload.card);

        if self.state.table_suit.is_none() {
            self.state.table_suit = Some(payload.card.suit);
        }

        // if everyone has placed a card, calculate who gets the score assigned and pick him as next player
        if self.state.cards_on_table.len() == 3 {
            let scoring_player = self.get_scoring_player();
            let score = self.get_total_score_of_cards_on_table();
            *self.scores.entry(scoring_player.clone()).or_insert(0) += score;
        }

        self.prepare_table_for_next_turn();
    }

    fn place_card(&mut self, card: &Card) {
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
        self.state.cards_on_table = HashMap::new();
        self.state.table_suit = None;
        self.state.current_player = self.player_to_player_map[&self.state.current_player].clone();
    }

    // TODO: implement those and write tests
    // pub fn should_switch(&self) -> bool {
    //
    // }
    
    // pub fn to_round_finished(self) -> RoundFinishedState {
    //
    // }
}


#[derive(Debug)]
pub struct RoundInProgressState {
    pub current_player: String,
    pub table_suit: Option<CardSuit>,
    pub cards_on_table: HashMap<String, Card>
}

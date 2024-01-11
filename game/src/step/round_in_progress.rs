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
        // remove card from user deck
        // place it on table

        // if there are 3 cards, calculate who gets the score assigned and pick him as next player
        // switch player
    }
}


#[derive(Debug)]
pub struct RoundInProgressState {
    pub current_player: String,
    pub table_suit: Option<CardSuit>,
    pub cards_on_table: HashMap<String, Card>
}

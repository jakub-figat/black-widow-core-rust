use std::collections::HashMap;
use crate::card::{Card, CardSuit};
use crate::error::GameResult;
use crate::payload::PlaceCardPayload;
use crate::step::GameStep;

impl GameStep<RoundInProgressState> {
    pub fn validate(&self, payload: &PlaceCardPayload) -> GameResult<()> {
        self.check_player_has_card(&payload.card, &self.state.current_player)

        // complex shit
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
    pub table_suit: CardSuit,
    pub cards_on_table: HashMap<String, Card>
}

use std::collections::HashMap;
use crate::card::{Card, CardSuit};
use crate::step::GameStep;

impl GameStep<RoundInProgressState> {
}


#[derive(Debug)]
pub struct RoundInProgressState {
    pub current_player: String,
    pub table_suit: CardSuit,
    pub cards_on_table: HashMap<String, Card>
}

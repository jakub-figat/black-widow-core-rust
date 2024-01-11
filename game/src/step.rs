use std::collections::{HashMap, HashSet};
use crate::card::{Card};

pub mod card_exchange;
pub mod round_in_progress;
pub mod round_finished;


#[derive(Debug)]
pub struct GameStep<T> {
    players: Vec<String>,
    scores: HashMap<String, usize>,
    player_decks: HashMap<String, HashSet<Card>>,
    state: T
}

use std::collections::HashMap;
use crate::card::{Card, CardSuit};

pub struct GameStep<T> {
    current_player: Option<String>,
    players: Vec<String>,
    scores: HashMap<String, usize>,
    pub player_decks: HashMap<String, Vec<Card>>,
    pub state: T
}

impl GameStep<CardExchangeState> {
    pub fn initialize_from_players(players: Vec<String>) -> GameStep<CardExchangeState> {
        GameStep {
            current_player: None,
            players: players.clone(),
            scores: HashMap::new(),
            player_decks: HashMap::new(),
            state: CardExchangeState::initialize_from_players(&players)
        }
    }
}

pub struct CardExchangeState {
    pub cards_to_exchange: HashMap<String, Vec<Card>>
}

pub struct RoundInProgressState {
    cards_on_table: HashMap<String, Card>,
    table_suit: Option<CardSuit>
}

pub struct RoundFinishedState {
    users_ready: HashMap<String, bool>
}

impl CardExchangeState {
    fn initialize_from_players(players: &Vec<String>) -> CardExchangeState {
        let mut cards_to_exchange = HashMap::new();
        for player in players {
            cards_to_exchange.entry(player.clone()).or_insert(vec![]);
        }
        CardExchangeState { cards_to_exchange }
    }
}

impl RoundInProgressState {}

impl RoundFinishedState {}
use crate::error::{GameError, GameResult};
use crate::game::GameState::CardExchange;
use crate::step::card_exchange::CardExchangeState;
use crate::step::round_finished::RoundFinishedState;
use crate::step::round_in_progress::RoundInProgressState;
use crate::step::GameStep;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Game {
    pub settings: GameSettings,
    pub players: Vec<String>,
    pub state: GameState,
    pub is_finished: bool,
}

impl Game {
    pub fn from_players(players: &[String], settings: GameSettings) -> GameResult<Game> {
        let number_of_players = players.len();
        if number_of_players < 3 || number_of_players > 4 {
            Err(GameError("Invalid number of players".to_string()))?
        }

        Ok(Game {
            settings,
            players: players.to_vec(),
            state: GameState::get_initial_state(players),
            is_finished: false,
        })
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GameSettings {
    #[serde(rename = "maxScore")]
    pub max_score: usize,
}

#[derive(Debug, Clone)]
pub enum GameState {
    CardExchange(GameStep<CardExchangeState>),
    RoundInProgress(GameStep<RoundInProgressState>),
    RoundFinished(GameStep<RoundFinishedState>),
}

impl GameState {
    fn get_initial_state(players: &[String]) -> GameState {
        CardExchange(GameStep::initialize_from_players(players))
    }
}

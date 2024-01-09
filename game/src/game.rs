use crate::step::{
    CardExchangeState, 
    RoundInProgressState,
    RoundFinishedState, 
    GameStep
};


pub struct Game {
    settings: GameSettings,
    pub game_state: GameState
}

// TODO later: validate that payload user is current user

impl Game {
    pub fn new(players: &[String], settings: GameSettings) -> Game {
        let number_of_players = players.len();
        if number_of_players < 3 || number_of_players > 4 {
            panic!("Invalid number of players");
        }

        let state = GameState::get_initial_state(players.to_vec());
        Game {
            settings,
            game_state: state
        }
    }
}

pub struct GameSettings {
    pub max_score: usize,
}

pub enum GameState {
    CardExchange(GameStep<CardExchangeState>),
    FirstRound(GameStep<RoundInProgressState>),
    RoundInProgress(GameStep<RoundInProgressState>),
    RoundFinished(GameStep<RoundFinishedState>)
}

impl GameState {
    fn get_initial_state(players: Vec<String>) -> GameState {
        GameState::CardExchange(GameStep::initialize_from_players(players))
    }
}

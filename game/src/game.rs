use crate::step::{
    CardExchangeState, 
    RoundInProgressState,
    RoundFinishedState, 
    GameStep
};


pub struct Game {
    settings: GameSettings,
    players: Vec<String>
}

// TODO later: validate that payload user is current user

impl Game {
    pub fn new(players: &[String], settings: GameSettings) -> Game {
        let number_of_players = players.len();
        if number_of_players < 3 || number_of_players > 4 {
            panic!("Invalid number of players");
        }

        Game {
            settings,
            players: players.to_vec()
        }
    }

    // pub fn play(&self, input_handler: Box<dyn Fn() -> String>) {
    //     let mut game_state = GameState::get_initial_state(self.players.to_vec());
    //     let json_string = input_handler();
    //
    //     TODO: finish state transition, input validation later
    //     loop {
    //         game_state = match game_state {
    //             GameState::CardExchange(exchange_step) => {
    //                 if exchange_step.should_switch() {
    //                     GameState::RoundInProgress(GameStep::from_card_exchange_step(exchange_step))
    //                 } else {
    //                     GameState::get_initial_state(self.players.to_vec())
    //                 }
    //             }
    //             GameState::FirstRound(round_step) => {
    //                 GameState::get_initial_state(self.players.to_vec())
    //             }
    //             GameState::RoundInProgress(round_step) => {
    //                 GameState::get_initial_state(self.players.to_vec())
    //             }
    //             GameState::RoundFinished(finished_step) => {
    //                 break
    //             }
    //         }
    //     }
    //     println!("Game finished");
    // }
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

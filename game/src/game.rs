use std::error::Error;
use crate::game::GameState::{CardExchange, RoundFinished, RoundInProgress};
use crate::r#trait::PayloadHandler;
use crate::step::{
    GameStep
};
use crate::step::card_exchange::CardExchangeState;
use crate::step::round_finished::RoundFinishedState;
use crate::step::round_in_progress::RoundInProgressState;


pub struct Game {
    settings: GameSettings,
    players: Vec<String>
}

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

    pub fn play(&self, input_handler: fn() -> (String, String), error_writer: fn(err: Box<dyn Error>)) {
        let mut game_state = GameState::get_initial_state(&self.players);
        loop {
            let (json_payload, player) = input_handler();

            game_state = match game_state {
                CardExchange(mut step) => {
                    step.handle_payload(&json_payload, &player, error_writer);
                    match step.should_switch() {
                        true => RoundInProgress(step.to_round_in_progress()),
                        false => CardExchange(step)
                    }
                },
                RoundInProgress(mut step) => {
                    step.handle_payload(&json_payload, &player, error_writer);
                    match step.should_switch() {
                        true => RoundFinished(step.to_round_finished()),
                        false => RoundInProgress(step)
                    }
                },
                RoundFinished(mut step) => {
                    step.handle_payload(&json_payload, &player, error_writer);
                    if step.game_finished(self.settings.max_score) {
                        break;
                    }
                    match step.should_switch() {
                        true => CardExchange(step.to_card_exchange()),
                        false => RoundFinished(step)
                    }
                }
            };
        }

        println!("Game finished");
    }
}

pub struct GameSettings {
    pub max_score: usize,
}

pub enum GameState {
    CardExchange(GameStep<CardExchangeState>),
    RoundInProgress(GameStep<RoundInProgressState>),
    RoundFinished(GameStep<RoundFinishedState>)
}


impl GameState {
    fn get_initial_state(players: &Vec<String>) -> GameState {
        CardExchange(GameStep::initialize_from_players(players))
    }
}

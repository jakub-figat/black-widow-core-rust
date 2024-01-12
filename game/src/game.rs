use std::error::Error;
use crate::game::GameState::{CardExchange, RoundFinished, RoundInProgress};
use crate::payload::{CardExchangePayload, PlaceCardPayload};
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

    pub fn play(&self, input_handler: fn() -> String, error_writer: fn(err: Box<dyn Error>)) {
        let mut game_state = GameState::get_initial_state(&self.players);
        loop {
            let json_payload = input_handler();

            game_state = match game_state {
                CardExchange(step) => {
                    Game::handle_card_exchange_step(step.clone(), &json_payload).unwrap_or_else(|error| {
                        error_writer(error);
                        CardExchange(step)
                    })
                },
                RoundInProgress(step) => {
                    Game::handle_card_round_in_progress_step(step.clone(), &json_payload).unwrap_or_else(|error| {
                        error_writer(error);
                        RoundInProgress(step)
                    })
                },
                RoundFinished(step) => {
                    if step.game_finished(self.settings.max_score) {
                        break;
                    }
                    Game::handle_round_finished_step(step.clone()).unwrap_or_else(|error| {
                        error_writer(error);
                        RoundFinished(step)
                    })
                }
            };
        }

        println!("Game finished");
    }

    fn handle_card_exchange_step(
        mut step: GameStep<CardExchangeState>, json_payload: &str
    ) -> Result<GameState, Box<dyn Error>> {
        let payload: CardExchangePayload = serde_json::from_str(json_payload)?;
        step.validate_payload(&payload, "1")?;
        step.dispatch_payload(&payload, "1");
        let state = match step.should_switch() {
            true => RoundInProgress(step.to_round_in_progress()),
            false => CardExchange(step)
        };

        Ok(state)
    }

    fn handle_card_round_in_progress_step(
        mut step: GameStep<RoundInProgressState>, json_payload: &str
    ) -> Result<GameState, Box<dyn Error>> {
        let payload: PlaceCardPayload = serde_json::from_str(json_payload)?;
        step.validate_payload(&payload)?;
        step.dispatch_payload(&payload);
        let state = match step.should_switch() {
            true => RoundFinished(step.to_round_in_progress()),
            false => RoundInProgress(step)
        };

        Ok(state)
    }

    fn handle_round_finished_step(mut step: GameStep<RoundFinishedState>) -> Result<GameState, Box<dyn Error>> {
        step.claim_readiness("1")?;
        let state = match step.should_switch() {
            true => CardExchange(step.to_round_in_progress()),
            false => RoundFinished(step)
        };

        Ok(state)
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

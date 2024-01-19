use std::error::Error;
use crate::error::GameError;
use crate::game::GameState::{CardExchange, RoundFinished, RoundInProgress};
use crate::r#trait::PayloadHandler;
use crate::step::{
    GameStep
};
use crate::step::card_exchange::CardExchangeState;
use crate::step::round_finished::RoundFinishedState;
use crate::step::round_in_progress::RoundInProgressState;


#[derive(Debug)]
pub struct Game {
    settings: GameSettings,
    pub players: Vec<String>,
    state: Option<GameState>
}

impl Game {
    pub fn from_players(players: &[String], settings: GameSettings) -> Game {
        let number_of_players = players.len();
        if number_of_players < 3 || number_of_players > 4 {
            panic!("Invalid number of players");
        }

        Game {
            settings,
            players: players.to_vec(),
            state: Some(GameState::get_initial_state(players))
        }
    }

    pub fn new_by_player(player: &str, settings: GameSettings) -> Game {
        let mut game = Game {
            settings,
            players: Vec::with_capacity(4),
            state: None
        };
        game.players.push(player.to_string());
        game
    }

    pub fn dispatch_payload(&mut self, payload: &str, player: &str) -> Result<(), Box<dyn Error>> {
        match self.state.take() {
            Some(state) => {
                self.state = Some(match state {
                    CardExchange(mut step) => {
                        step.handle_payload(&payload, &player)?;
                        match step.should_switch() {
                            true => RoundInProgress(step.to_round_in_progress()),
                            false => CardExchange(step)
                        }
                    },
                    RoundInProgress(mut step) => {
                        step.handle_payload(&payload, &player)?;
                        match step.should_switch() {
                            true => RoundFinished(step.to_round_finished()),
                            false => RoundInProgress(step)
                        }
                    },
                    RoundFinished(mut step) => {
                        step.handle_payload(&payload, &player)?;
                        if step.game_finished(self.settings.max_score) {
                            println!("game finished!") // TODO
                        }
                        match step.should_switch() {
                            true => CardExchange(step.to_card_exchange()),
                            false => RoundFinished(step)
                        }
                    }
                });

                Ok(())
            }
            None => Err(GameError::InvalidAction(
                "Cannot dispatch payload, game state is not initialized".to_string()
            ))?
        }
    }
}

#[derive(Debug)]
pub struct GameSettings {
    pub max_score: usize,
}

#[derive(Debug)]
pub enum GameState {
    CardExchange(GameStep<CardExchangeState>),
    RoundInProgress(GameStep<RoundInProgressState>),
    RoundFinished(GameStep<RoundFinishedState>)
}


impl GameState {
    fn get_initial_state(players: &[String]) -> GameState {
        CardExchange(GameStep::initialize_from_players(players))
    }
}

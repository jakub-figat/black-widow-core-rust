use std::collections::HashSet;
use crate::error::{GameError, GameResult};
use crate::step::card_exchange::CardExchangeState;
use crate::step::GameStep;


impl GameStep<RoundFinishedState> {
    pub fn claim_readiness(&mut self, player: &str) -> GameResult<()> {
        if self.state.players_ready.contains(player) {
            Err(
                GameError::InvalidAction(
                    format!("Player {} has already claimed readiness", player)
                )
            )?
        }

        self.state.players_ready.insert(player.to_string());
        Ok(())
    }

    pub fn should_switch(&self) -> bool {
        self.state.players_ready.len() == self.players.len()
    }

    pub fn game_finished(&self) -> bool {
        self.scores.iter().max_by_key(|(_, &score)| score).unwrap().1.clone() >= 100
    }

    pub fn to_card_exchange(self) -> GameStep<CardExchangeState> {
        GameStep {
            players: self.players,
            player_to_player_map: self.player_to_player_map,
            scores: self.scores,
            player_decks: self.player_decks,
            state: CardExchangeState::new()
        }
    }

}

pub struct RoundFinishedState {
    pub players_ready: HashSet<String>
}

impl RoundFinishedState {}
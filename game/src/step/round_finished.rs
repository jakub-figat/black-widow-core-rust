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

#[derive(Debug)]
pub struct RoundFinishedState {
    pub players_ready: HashSet<String>
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::helper::get_player_to_player_map;
    use super::*;

    fn get_step() -> GameStep<RoundFinishedState> {
        let players = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ];
        GameStep {
            players: players.clone(),
            player_to_player_map: get_player_to_player_map(&players),
            scores: HashMap::new(),
            player_decks: HashMap::new(),
            state: RoundFinishedState {players_ready: HashSet::new()}
        }
    }

    #[test]
    fn claim_readiness() {
        let mut step = get_step();
        let result = step.claim_readiness("1");

        assert!(result.is_ok());
        assert_eq!(step.state.players_ready.len(), 1);
    }

    #[test]
    fn claim_readiness_when_already_claimed() {
        let mut step = get_step();
        step.state.players_ready.insert("1".to_string());
        let result = step.claim_readiness("1");

        assert_eq!(result, Err(GameError::InvalidAction("Player 1 has already claimed readiness".to_string())));
        assert_eq!(step.state.players_ready.len(), 1);
    }

    #[test]
    fn game_finished_when_one_of_players_has_score_equal_or_more_than_100() {
        let mut step = get_step();
        step.scores.insert("1".to_string(), 100);

        assert!(step.game_finished());
    }

    #[test]
    fn game_finished_when_one_of_players_has_score_less_than_100() {
        let mut step = get_step();
        step.scores.insert("1".to_string(), 99);

        assert!(!step.game_finished());
    }
}

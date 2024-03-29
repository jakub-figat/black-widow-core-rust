use crate::helper::get_starting_player_decks;
use crate::payload::ClaimReadinessPayload;
use crate::step::card_exchange::CardExchangeState;
use crate::step::GameStep;
use std::collections::HashMap;

impl GameStep<RoundFinishedState> {
    pub fn handle_payload(&mut self, payload: &ClaimReadinessPayload, player: &str) {
        self.state
            .players_ready
            .insert(player.to_string(), payload.ready);
    }

    pub fn should_switch(&self) -> bool {
        self.state
            .players_ready
            .values()
            .map(|&ready| ready == true)
            .len()
            == self.players.len()
    }

    pub fn game_finished(&self, max_score: usize) -> bool {
        self.scores
            .iter()
            .max_by_key(|(_, &score)| score)
            .unwrap()
            .1
            .clone()
            >= max_score
    }

    pub fn to_card_exchange(self) -> GameStep<CardExchangeState> {
        GameStep {
            players: self.players.clone(),
            player_to_player_map: self.player_to_player_map,
            scores: self.scores,
            player_decks: get_starting_player_decks(&self.players),
            state: CardExchangeState::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoundFinishedState {
    pub players_ready: HashMap<String, bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::get_player_to_player_map;
    use std::collections::HashMap;

    fn get_step() -> GameStep<RoundFinishedState> {
        let players = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        GameStep {
            players: players.clone(),
            player_to_player_map: get_player_to_player_map(&players),
            scores: HashMap::new(),
            player_decks: HashMap::new(),
            state: RoundFinishedState {
                players_ready: HashMap::new(),
            },
        }
    }

    #[test]
    fn claim_readiness() {
        let mut step = get_step();
        let payload = ClaimReadinessPayload { ready: true };
        step.handle_payload(&payload, "1");

        assert_eq!(step.state.players_ready["1"], true);
    }

    #[test]
    fn claim_readiness_when_already_claimed() {
        let mut step = get_step();
        step.state.players_ready.insert("1".to_string(), true);

        let payload = ClaimReadinessPayload { ready: false };
        step.handle_payload(&payload, "1");

        assert_eq!(step.state.players_ready["1"], false);
    }

    #[test]
    fn game_finished_when_one_of_players_has_score_equal_or_more_than_100() {
        let mut step = get_step();
        step.scores.insert("1".to_string(), 100);

        assert!(step.game_finished(100));
    }

    #[test]
    fn game_finished_when_one_of_players_has_score_less_than_100() {
        let mut step = get_step();
        step.scores.insert("1".to_string(), 99);

        assert!(!step.game_finished(100));
    }
}

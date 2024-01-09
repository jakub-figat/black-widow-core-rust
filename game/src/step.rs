use std::collections::HashMap;
use crate::card::{Card, CardSuit};
use crate::error::GameError;
use crate::helper::get_players_cards_hashmap;
use crate::payload::CardExchangePayload;

#[derive(Debug)]
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
            current_player: Some(players[0].clone()),
            players: players.clone(),
            scores: HashMap::new(),
            player_decks: get_players_cards_hashmap(&players),
            state: CardExchangeState::initialize_from_players(&players)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CardExchangeState {
    pub cards_to_exchange: HashMap<String, Vec<Card>>
}

impl CardExchangeState {
    fn initialize_from_players(players: &[String]) -> CardExchangeState {
        CardExchangeState { cards_to_exchange: get_players_cards_hashmap(&players) }
    }

    pub fn validate_payload(
        &self,
        payload: &CardExchangePayload,
        step: &GameStep<CardExchangeState>
    ) -> Result<(), GameError> {
        let player = step.current_player.as_ref().unwrap();
        if self.cards_to_exchange.get(player).unwrap().len() != 0 {
            Err(
                GameError::InvalidAction(
                    format!("Player {} has already declared cards for exchange", player)
                )
            )?
        }

        for card in &payload.cards_to_exchange {
            if !&step.player_decks.get(player).unwrap().contains(card) {
                Err(
                    GameError::InvalidAction(
                        format!("Player {} does not have a card", player)
                    )
                )?
            }
        }

        Ok(())
    }
}

pub struct RoundInProgressState {
    cards_on_table: HashMap<String, Card>,
    table_suit: Option<CardSuit>
}

impl RoundInProgressState {}

pub struct RoundFinishedState {
    users_ready: HashMap<String, bool>
}

impl RoundFinishedState {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_exchange_state_from_players() {
        let players = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ];
        let state = CardExchangeState::initialize_from_players(&players);

        assert_eq!(
            state,
            CardExchangeState {
                cards_to_exchange: HashMap::from(
                    [
                        ("1".to_string(), vec![]),
                        ("2".to_string(), vec![]),
                        ("3".to_string(), vec![]),
                    ]
                )
            }
        );
    }

    #[test]
    fn card_exchange_state_validate_payload_returns_error_when_player_does_not_have_a_card() {
        let players = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ];
        let mut step = GameStep::initialize_from_players(players);


        let cards = vec![
            Card::new(CardSuit::Spade, 2),
            Card::new(CardSuit::Spade, 3),
            Card::new(CardSuit::Spade, 4)
        ];

        let payload = CardExchangePayload::from_cards(&cards).unwrap();

        assert_eq!(
            step.state.validate_payload(&payload, &step),
            Err(
                GameError::InvalidAction("Player 1 does not have a card".to_string())
            )
        );
    }

    #[test]
    fn card_exchange_state_validate_payload_returns_error_when_player_already_placed_cards_for_exchange() {
        let players = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ];
        let mut step = GameStep::initialize_from_players(players);
        let cards = vec![
            Card::new(CardSuit::Spade, 2),
            Card::new(CardSuit::Spade, 3),
            Card::new(CardSuit::Spade, 4)
        ];

        step.state.cards_to_exchange.insert("1".to_string(), cards.clone());
        let payload = CardExchangePayload::from_cards(&cards).unwrap();

        assert_eq!(
            step.state.validate_payload(&payload, &step),
            Err(
                GameError::InvalidAction("Player 1 has already declared cards for exchange".to_string())
            )
        )
    }
}
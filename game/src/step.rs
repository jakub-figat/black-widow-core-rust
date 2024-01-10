use std::collections::{HashMap, HashSet};
use crate::card::{Card, CardSuit};
use crate::error::GameError;
use crate::payload::CardExchangePayload;

#[derive(Debug)]
pub struct GameStep<T> {
    current_player: String,
    players: Vec<String>,
    scores: HashMap<String, usize>,
    player_decks: HashMap<String, HashSet<Card>>,
    state: T
}

impl GameStep<CardExchangeState> {
    pub fn initialize_from_players(players: Vec<String>) -> GameStep<CardExchangeState> {
        GameStep {
            current_player: players[0].clone(),
            players: players.clone(),
            scores: HashMap::new(),
            player_decks: HashMap::from_iter(
                players.iter()
                    .cloned()
                    .map(|player| (player, HashSet::new()))
                    .collect::<HashMap<_, _>>()
            ),
            state: CardExchangeState::new()
        }
    }

    pub fn validate_payload(
        &self,
        payload: &CardExchangePayload,
    ) -> Result<(), GameError> {
        let player = &self.current_player;

        if self.state.cards_to_exchange.get(player).is_some() {
            Err(
                GameError::InvalidAction(
                    format!("Player {} has already declared cards for exchange", player)
                )
            )?
        }

        for card in &payload.cards_to_exchange {
            if !&self.player_decks.get(player).unwrap().contains(card) {
                Err(
                    GameError::InvalidAction(
                        format!("Player {} does not have a card", player)
                    )
                )?
            }
        }

        Ok(())
    }

    pub fn dispatch_payload(
        &mut self,
        payload: &CardExchangePayload,
    ) {
        self.state.cards_to_exchange.insert(
            self.current_player.clone(), payload.cards_to_exchange.clone()
        );
    }

    pub fn should_switch(&self) -> bool {
        self.players.len() == self.state.cards_to_exchange.len()
    }
}

impl GameStep<RoundInProgressState> {
    pub fn from_card_exchange_step(
        card_exchange_step: GameStep<CardExchangeState>
    ) -> GameStep<RoundInProgressState> {
        GameStep {
            current_player: card_exchange_step.current_player,
            players: card_exchange_step.players,
            scores: card_exchange_step.scores,
            player_decks: GameStep::exchange_cards_between_players(
                card_exchange_step.player_decks, &card_exchange_step.state.cards_to_exchange
            ),
            state: RoundInProgressState::new()
        }
    }

    fn exchange_cards_between_players(
        mut player_decks: HashMap<String, HashSet<Card>>,
        cards_for_exchange: &HashMap<String, HashSet<Card>>
    ) -> HashMap<String, HashSet<Card>> {
        let players_vec: Vec<_> = cards_for_exchange.keys().into_iter().collect();

        for (index, (from_player, cards)) in cards_for_exchange.iter().enumerate() {
            let to_player = players_vec[index + 1 % &players_vec.len()];
            for card in cards {
                player_decks.get_mut(from_player).unwrap().remove(card);
                player_decks.get_mut(to_player).unwrap().insert(card.clone());
            }
        }
        player_decks
    }
}

#[derive(Debug, PartialEq)]
pub struct CardExchangeState {
    pub cards_to_exchange: HashMap<String, HashSet<Card>>
}

impl CardExchangeState {
    pub fn new() -> CardExchangeState {
        CardExchangeState {cards_to_exchange: HashMap::new()}
    }
}

pub struct RoundInProgressState {
    cards_on_table: HashMap<String, Card>,
    table_suit: Option<CardSuit>
}

impl RoundInProgressState {
    pub fn new() -> RoundInProgressState {
        RoundInProgressState {
            cards_on_table: HashMap::new(), table_suit: None
        }
    }
}

pub struct RoundFinishedState {
    users_ready: HashMap<String, bool>
}

impl RoundFinishedState {}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_players() -> Vec<String> {
        vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string()
        ]
    }

    #[test]
    fn card_exchange_step_validate_payload_returns_error_when_player_does_not_have_a_card() {
        let players = get_players();
        let step = GameStep::initialize_from_players(players);


        let cards = HashSet::from(
            [
                Card::new(CardSuit::Spade, 2),
                Card::new(CardSuit::Spade, 3),
                Card::new(CardSuit::Spade, 4)
            ]
        );

        let payload = CardExchangePayload::from_cards(&cards).unwrap();

        assert_eq!(
            step.validate_payload(&payload),
            Err(
                GameError::InvalidAction("Player 1 does not have a card".to_string())
            )
        );
    }

    #[test]
    fn card_exchange_step_validate_payload_returns_error_when_player_already_placed_cards_for_exchange() {
        let players = get_players();
        let mut step = GameStep::initialize_from_players(players);
        let cards = HashSet::from(
            [
                Card::new(CardSuit::Spade, 2),
                Card::new(CardSuit::Spade, 3),
                Card::new(CardSuit::Spade, 4)
            ]
        );

        step.state.cards_to_exchange.insert("1".to_string(), cards.clone());
        let payload = CardExchangePayload::from_cards(&cards).unwrap();

        assert_eq!(
            step.validate_payload(&payload),
            Err(
                GameError::InvalidAction("Player 1 has already declared cards for exchange".to_string())
            )
        )
    }

    #[test]
    fn card_exchange_step_player_1_dispatch_payload() {
        let players = get_players();
        let cards = HashSet::from(
            [
                Card::new(CardSuit::Spade, 2),
                Card::new(CardSuit::Spade, 3),
                Card::new(CardSuit::Spade, 4),
            ]
        );
        let payload = CardExchangePayload::from_cards(&cards).unwrap();
        let mut step = GameStep::initialize_from_players(players);
        step.dispatch_payload(&payload);

        assert_eq!(
            step.state.cards_to_exchange,
            HashMap::from(
                [
                    ("1".to_string(), cards),
                ]
            )
        );
    }

    // TODO: tests whether cards are exchanged correctly within state transition
}
use crate::card::Card;
use crate::card::CardSuit::Club;
use crate::error::{GameError, GameResult};
use crate::helper::{
    get_player_to_player_map, get_starting_player_decks, pick_player_with_starting_card,
};
use crate::payload::CardExchangePayload;
use crate::r#trait::PayloadHandler;
use crate::step::round_in_progress::RoundInProgressState;
use crate::step::GameStep;
use std::collections::{HashMap, HashSet};
use std::error::Error;

impl GameStep<CardExchangeState> {
    pub fn initialize_from_players(players: &[String]) -> GameStep<CardExchangeState> {
        GameStep {
            players: players.to_vec(),
            player_to_player_map: get_player_to_player_map(&players),
            scores: HashMap::new(),
            player_decks: get_starting_player_decks(&players),
            state: CardExchangeState::new(),
        }
    }

    pub fn empty_from_players(players: &[String]) -> GameStep<CardExchangeState> {
        GameStep {
            players: players.to_vec(),
            player_to_player_map: get_player_to_player_map(&players),
            scores: HashMap::new(),
            player_decks: HashMap::from_iter(
                players
                    .iter()
                    .cloned()
                    .map(|player| (player, HashSet::new()))
                    .collect::<HashMap<_, _>>(),
            ),
            state: CardExchangeState::new(),
        }
    }

    pub fn should_switch(&self) -> bool {
        self.players.len() == self.state.cards_to_exchange.len()
    }

    pub fn to_round_in_progress(mut self) -> GameStep<RoundInProgressState> {
        self.exchange_cards_between_players();

        let (player, starting_card) = pick_player_with_starting_card(&self.player_decks).unwrap();
        self.player_decks
            .get_mut(&player)
            .unwrap()
            .remove(&starting_card);

        let state = RoundInProgressState {
            current_player: self.player_to_player_map.get(&player).unwrap().to_string(),
            table_suit: Some(Club),
            cards_on_table: HashMap::from([(player, starting_card)]),
        };

        GameStep {
            players: self.players,
            player_to_player_map: self.player_to_player_map,
            scores: self.scores,
            player_decks: self.player_decks,
            state,
        }
    }

    fn exchange_cards_between_players(&mut self) {
        let player_decks = &mut self.player_decks;
        for (from_player, to_player) in &self.player_to_player_map {
            for card in &self.state.cards_to_exchange[from_player] {
                player_decks.get_mut(from_player).unwrap().remove(card);
                player_decks
                    .get_mut(to_player)
                    .unwrap()
                    .insert(card.clone());
            }
        }
    }
}

impl PayloadHandler<'_, CardExchangePayload> for GameStep<CardExchangeState> {
    fn validate_payload(&self, payload: &CardExchangePayload, player: &str) -> GameResult<()> {
        if self.state.cards_to_exchange.get(player).is_some() {
            Err(GameError(format!(
                "Player {} has already declared cards for exchange",
                player
            )))?
        }

        for card in &payload.cards_to_exchange {
            self.validate_player_has_card(card, player)?
        }

        Ok(())
    }

    fn dispatch_payload(&mut self, payload: &CardExchangePayload, player: &str) {
        self.state
            .cards_to_exchange
            .insert(player.to_string(), payload.cards_to_exchange.clone());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CardExchangeState {
    pub cards_to_exchange: HashMap<String, HashSet<Card>>,
}

impl CardExchangeState {
    pub fn new() -> CardExchangeState {
        CardExchangeState {
            cards_to_exchange: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardSuit::Spade;

    fn get_players() -> Vec<String> {
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    }

    fn get_decks_of_three_cards() -> Vec<HashSet<Card>> {
        vec![
            HashSet::from([Card::new(Spade, 2), Card::new(Club, 3), Card::new(Spade, 4)]),
            HashSet::from([
                Card::new(Spade, 5),
                Card::new(Spade, 6),
                Card::new(Spade, 7),
            ]),
            HashSet::from([
                Card::new(Spade, 8),
                Card::new(Spade, 9),
                Card::new(Spade, 10),
            ]),
        ]
    }

    fn insert_decks_of_cards(
        step: &mut GameStep<CardExchangeState>,
        decks_of_cards: &Vec<HashSet<Card>>,
    ) {
        step.player_decks
            .insert("1".to_string(), decks_of_cards[0].clone());
        step.player_decks
            .insert("2".to_string(), decks_of_cards[1].clone());
        step.player_decks
            .insert("3".to_string(), decks_of_cards[2].clone());
    }

    #[test]
    fn validate_payload_returns_error_when_player_already_placed_cards_for_exchange() {
        let players = get_players();
        let mut step = GameStep::empty_from_players(&players);
        let cards = HashSet::from([
            Card::new(Spade, 2),
            Card::new(Spade, 3),
            Card::new(Spade, 4),
        ]);

        step.state
            .cards_to_exchange
            .insert("1".to_string(), cards.clone());
        let payload = CardExchangePayload::from_cards(&cards).unwrap();

        assert_eq!(
            step.validate_payload(&payload, &players[0]),
            Err(GameError(
                "Player 1 has already declared cards for exchange".to_string()
            ))
        )
    }

    #[test]
    fn player_1_dispatch_payload() {
        let players = get_players();
        let cards = HashSet::from([
            Card::new(Spade, 2),
            Card::new(Spade, 3),
            Card::new(Spade, 4),
        ]);
        let payload = CardExchangePayload::from_cards(&cards).unwrap();
        let mut step = GameStep::empty_from_players(&players);
        step.dispatch_payload(&payload, &players[0]);

        assert_eq!(
            step.state.cards_to_exchange,
            HashMap::from([("1".to_string(), cards),])
        );
    }

    #[test]
    fn should_switch_returns_true_when_all_players_have_placed_their_cards() {
        let players = get_players();
        let vec_of_cards = get_decks_of_three_cards();

        let mut step = GameStep::empty_from_players(&players);
        insert_decks_of_cards(&mut step, &vec_of_cards);

        step.state
            .cards_to_exchange
            .insert("1".to_string(), vec_of_cards[0].clone());
        step.state
            .cards_to_exchange
            .insert("2".to_string(), vec_of_cards[1].clone());
        step.state
            .cards_to_exchange
            .insert("3".to_string(), vec_of_cards[2].clone());

        assert!(step.should_switch())
    }

    #[test]
    fn exchange_cards_between_players() {
        let players = get_players();
        let vec_of_cards = get_decks_of_three_cards();

        let mut step = GameStep::empty_from_players(&players);
        insert_decks_of_cards(&mut step, &vec_of_cards);

        step.state
            .cards_to_exchange
            .insert("1".to_string(), vec_of_cards[0].clone());
        step.state
            .cards_to_exchange
            .insert("2".to_string(), vec_of_cards[1].clone());
        step.state
            .cards_to_exchange
            .insert("3".to_string(), vec_of_cards[2].clone());
        step.exchange_cards_between_players();

        let expected_decks = HashMap::from([
            ("1".to_string(), HashSet::from(vec_of_cards[2].clone())),
            ("2".to_string(), HashSet::from(vec_of_cards[0].clone())),
            ("3".to_string(), HashSet::from(vec_of_cards[1].clone())),
        ]);
        assert_eq!(step.player_decks, expected_decks);
    }

    #[test]
    fn to_round_in_progress() {
        let players = get_players();
        let vec_of_cards = get_decks_of_three_cards();

        let mut step = GameStep::empty_from_players(&players);
        insert_decks_of_cards(&mut step, &vec_of_cards);

        step.state
            .cards_to_exchange
            .insert("1".to_string(), vec_of_cards[0].clone());
        step.state
            .cards_to_exchange
            .insert("2".to_string(), vec_of_cards[1].clone());
        step.state
            .cards_to_exchange
            .insert("3".to_string(), vec_of_cards[2].clone());

        let round_in_progress_step = step.to_round_in_progress();

        // club 3 card starts in player 1 deck
        // during the exchange, it will get passed to player 2
        // player 2 placed club 3 automatically due to game rules, so player 3 is to_round_in_progress
        assert_eq!(round_in_progress_step.state.current_player, "3".to_string());
        assert_eq!(round_in_progress_step.state.table_suit, Some(Club));
    }
}

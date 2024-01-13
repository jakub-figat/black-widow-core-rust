use std::collections::HashSet;
use serde::Deserialize;
use crate::card::Card;
use crate::error::{GameError, GameResult};


#[derive(Debug, PartialEq, Deserialize)]
pub struct CardExchangePayload {
    pub cards_to_exchange: HashSet<Card>
}

impl CardExchangePayload {
    pub fn from_cards(cards: &HashSet<Card>) -> GameResult<CardExchangePayload> {
        if cards.len() != 3 {
            Err(
                GameError::InvalidPayload(
                    "CardExchangePayload cards require passing exactly 3 cards".to_string()
                )
            )?
        }

        Ok(CardExchangePayload {cards_to_exchange: cards.clone()})
    }
}


#[derive(Deserialize)]
pub struct PlaceCardPayload {
    pub card: Card
}

#[derive(Deserialize)]
pub struct ClaimReadinessPayload {
    pub ready: bool
}

#[cfg(test)]
mod tests {
    use crate::card::CardSuit;
    use super::*;

    #[test]
    fn card_exchange_payload_from_cards_panics_when_there_arent_3_cards() {
        let cards = HashSet::from(
            [
                Card::new(CardSuit::Spade, 2),
                Card::new(CardSuit::Spade, 3),
                Card::new(CardSuit::Spade, 4),
                Card::new(CardSuit::Spade, 5)
            ]
        );

        let result = CardExchangePayload::from_cards(&cards);
        assert_eq!(
            result,
            Err(
                GameError::InvalidPayload(
                    "CardExchangePayload cards require passing exactly 3 cards".to_string()
                )
            )
        )
    }
}
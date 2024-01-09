use std::collections::HashSet;
use crate::card::Card;
use crate::error::{GameError, GameResult};


#[derive(Debug, PartialEq)]
pub struct CardExchangePayload {
    pub cards_to_exchange: Vec<Card>
}

impl CardExchangePayload {
    pub fn from_cards(cards: &[Card]) -> GameResult<CardExchangePayload> {
        if cards.len() != 3 {
            Err(
                GameError::InvalidPayload(
                    "CardExchangePayload cards require passing exactly 3 cards".to_string()
                )
            )?
        }

        let mut cards_unique: HashSet<&Card> = HashSet::from_iter(cards.iter());
        if cards_unique.len() != cards.len() {
            Err(GameError::InvalidPayload("CardExchangePayload cards must be unique!".to_string()))?
        }

        Ok(CardExchangePayload {cards_to_exchange: cards.to_vec()})
    }
}

#[cfg(test)]
mod tests {
    use crate::card::CardSuit;
    use super::*;

    #[test]
    fn card_exchange_payload_from_cards_panics_when_there_arent_3_cards() {
        let cards = vec![
            Card::new(CardSuit::Spade, 2),
            Card::new(CardSuit::Spade, 3),
            Card::new(CardSuit::Spade, 4),
            Card::new(CardSuit::Spade, 5)
        ];

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

    #[test]
    fn card_exchange_payload_from_cards_panics_when_cards_arent_unique() {
        let cards = vec![
            Card::new(CardSuit::Spade, 2),
            Card::new(CardSuit::Spade, 3),
            Card::new(CardSuit::Spade, 3)
        ];

        let result = CardExchangePayload::from_cards(&cards);
        assert_eq!(
            result,
            Err(
                GameError::InvalidPayload(
                    "CardExchangePayload cards must be unique!".to_string()
                )
            )
        );
    }
}
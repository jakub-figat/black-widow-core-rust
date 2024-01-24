use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use ts_rs::TS;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize, TS)]
pub enum CardSuit {
    #[serde(rename = "SPADE")]
    Spade,
    #[serde(rename = "CLUB")]
    Club,
    #[serde(rename = "HEART")]
    Heart,
    #[serde(rename = "DIAMOND")]
    Diamond,
}

impl Display for CardSuit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let name = match self {
            CardSuit::Spade => "SPADE",
            CardSuit::Club => "CLUB",
            CardSuit::Heart => "HEART",
            CardSuit::Diamond => "DIAMOND",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize, TS)]
pub struct Card {
    pub suit: CardSuit,
    pub value: usize,
    pub score: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}_{}", &self.suit, self.value.to_string())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Card {
    pub fn new(suit: CardSuit, value: usize) -> Result<Card, String> {
        if value > 14 {
            Err("Card value cannot be greater than 14!")?
        }

        let score = match suit {
            CardSuit::Heart => 1,
            CardSuit::Spade => match value {
                12 => 13,
                13 => 10,
                14 => 7,
                _ => 0,
            },
            _ => 0,
        };

        Ok(Card { suit, value, score })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ace_is_greater_than_king() {
        let ace = Card::new(CardSuit::Spade, 14);
        let king = Card::new(CardSuit::Heart, 13);
        assert!(ace > king);
    }

    #[test]
    fn hearts_get_assigned_1_score() {
        let heart_ace = Card::new(CardSuit::Heart, 14).unwrap();
        let heart_2 = Card::new(CardSuit::Heart, 2).unwrap();
        let spade_2 = Card::new(CardSuit::Spade, 2).unwrap();
        assert_eq!(heart_ace.score, 1);
        assert_eq!(heart_2.score, 1);
        assert_eq!(spade_2.score, 0);
    }

    #[test]
    fn cannot_create_card_with_value_greater_than_14() {
        assert!(Card::new(CardSuit::Spade, 15).is_err());
    }
}

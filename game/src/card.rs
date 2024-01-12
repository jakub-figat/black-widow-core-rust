use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize)]
pub enum CardSuit {
    Spade,
    Club,
    Heart,
    Diamond
}

impl Display for CardSuit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let name = match self {
            CardSuit::Spade => "SPADE",
            CardSuit::Club => "CLUB",
            CardSuit::Heart => "HEART",
            CardSuit::Diamond => "DIAMOND"
        };
        write!(f, "{}", name)
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize)]
pub struct Card {
    pub suit: CardSuit,
    pub value: usize,
    pub score: usize
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let temp_other = self.value.to_string();
;        let value = match self.value {
            11 => "JACK",
            12 => "QUEEN",
            13 => "KING",
            14 => "ACE",
            other => temp_other.as_str()
        };
        write!(f, "{}_{}", &self.suit, value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Card {
    pub fn new(suit: CardSuit, value: usize) -> Card {
        if value > 14 {
            panic!("Card value cannot be greater than 14!");
        }

        let score = match suit {
            CardSuit::Heart => 1,
            CardSuit::Spade => {
                match value {
                    12 => 13,
                    13 => 10,
                    14 => 7,
                    _ => 0
                }
            },
            _ => 0
        };

        Card {suit, value ,score}
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
        let heart_ace = Card::new(CardSuit::Heart, 14);
        let heart_2 = Card::new(CardSuit::Heart, 2);
        let spade_2 = Card::new(CardSuit::Spade, 2);
        assert_eq!(heart_ace.score, 1);
        assert_eq!(heart_2.score, 1);
        assert_eq!(spade_2.score, 0);
    }

    #[test]
    #[should_panic(expected = "Card value cannot be greater than 14!")]
    fn cannot_create_card_with_value_greater_than_14() {
        Card::new(CardSuit::Spade, 15);
    }
}
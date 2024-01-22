use crate::card::Card;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Deserialize)]
pub struct CardExchangePayload {
    pub cards_to_exchange: HashSet<Card>,
}

#[derive(Deserialize)]
pub struct PlaceCardPayload {
    pub card: Card,
}

#[derive(Deserialize)]
pub struct ClaimReadinessPayload {
    pub ready: bool,
}

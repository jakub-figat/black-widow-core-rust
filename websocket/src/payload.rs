use game::CardSuit;
use serde::Deserialize;
use std::collections::HashSet;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Deserialize, TS)]
#[ts(export)]
#[serde(tag = "action")]
pub enum WebSocketPayload {
    #[serde(rename = "listLobbies")]
    ListLobbies,
    #[serde(rename = "getLobbyDetails")]
    GetLobbyDetails(IdPayload),
    #[serde(rename = "createLobby")]
    CreateLobby(CreateLobbyPayload),
    #[serde(rename = "joinLobby")]
    JoinLobby(IdPayload),
    #[serde(rename = "quitLobby")]
    QuitLobby(IdPayload),
    #[serde(rename = "listGames")]
    ListGames,
    #[serde(rename = "getGameDetails")]
    GetGameDetails(IdPayload),
    #[serde(rename = "cardExchangeMove")]
    CardExchangeMove(CardExchangePayload),
    #[serde(rename = "placeCardMove")]
    PlaceCardMove(PlaceCardPayload),
    #[serde(rename = "claimReadinessMove")]
    ClaimReadinessMove(ClaimReadinessPayload),
    #[serde(rename = "quitGame")]
    QuitGame(IdPayload),
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct IdPayload {
    pub(crate) id: Uuid,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CreateLobbyPayload {
    #[serde(rename = "maxPlayers")]
    pub(crate) max_players: usize,
    #[serde(rename = "maxScore")]
    pub(crate) max_score: usize,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CardExchangePayload {
    pub(crate) id: Uuid,
    #[serde(rename = "cardsToExchange")]
    pub(crate) cards_to_exchange: HashSet<InputCard>,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct PlaceCardPayload {
    pub(crate) id: Uuid,
    pub(crate) card: InputCard,
}

#[derive(Deserialize, TS)]
#[ts(export)]
pub struct ClaimReadinessPayload {
    pub(crate) id: Uuid,
    pub(crate) ready: bool,
}

#[derive(PartialEq, Eq, Hash, Deserialize, TS)]
#[ts(export)]
pub struct InputCard {
    pub(crate) suit: CardSuit,
    pub(crate) value: usize,
}

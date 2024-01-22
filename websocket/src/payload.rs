use game::Card;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
#[serde(tag = "action")]
pub(crate) enum WebSocketPayload {
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

#[derive(Deserialize)]
pub(crate) struct IdPayload {
    pub(crate) id: String,
}

#[derive(Deserialize)]
pub(crate) struct CreateLobbyPayload {
    #[serde(rename = "maxPlayers")]
    pub(crate) max_players: usize,
    #[serde(rename = "maxScore")]
    pub(crate) max_score: usize,
}

#[derive(Deserialize)]
pub(crate) struct CardExchangePayload {
    pub(crate) id: String,
    #[serde(rename = "cardsToExchange")]
    pub(crate) cards_to_exchange: HashSet<Card>,
}

#[derive(Deserialize)]
pub(crate) struct PlaceCardPayload {
    pub(crate) id: String,
    pub(crate) card: Card,
}

#[derive(Deserialize)]
pub(crate) struct ClaimReadinessPayload {
    pub(crate) id: String,
    pub(crate) ready: bool,
}

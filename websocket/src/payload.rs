use game::Card;
use serde::de;
use serde::{Deserialize, Deserializer};
use std::collections::HashSet;
use std::str::FromStr;
use uuid::Uuid;

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
    #[serde(deserialize_with = "from_uuid")]
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
    #[serde(deserialize_with = "from_uuid")]
    pub(crate) id: String,
    #[serde(rename = "cardsToExchange")]
    pub(crate) cards_to_exchange: HashSet<Card>,
}

#[derive(Deserialize)]
pub(crate) struct PlaceCardPayload {
    #[serde(deserialize_with = "from_uuid")]
    pub(crate) id: String,
    pub(crate) card: Card,
}

#[derive(Deserialize)]
pub(crate) struct ClaimReadinessPayload {
    #[serde(deserialize_with = "from_uuid")]
    pub(crate) id: String,
    pub(crate) ready: bool,
}

fn from_uuid<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let uuid_string = Deserialize::deserialize(deserializer)?;
    match Uuid::from_str(uuid_string) {
        Ok(uuid) => Ok(uuid.to_string()),
        Err(error) => Err(de::Error::custom(error.to_string())),
    }
}

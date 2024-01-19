use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct WebSocketPayload {
    pub(crate) action: WebSocketAction,
    #[serde(default)]
    payload: String
}

#[derive(Deserialize)]
pub(crate) enum WebSocketAction {
    #[serde(rename = "listGames")]
    ListGames,
    #[serde(rename = "getGameDetails")]
    GetGameDetails,
    #[serde(rename = "startGame")]
    StartGame,
    #[serde(rename = "joinGame")]
    JoinGame,
    #[serde(rename = "gameMove")]
    GameMove
}
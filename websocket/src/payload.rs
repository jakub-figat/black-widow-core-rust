use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum WebSocketAction {
    #[serde(rename = "listLobbies")]
    ListLobbies,
    #[serde(rename = "getLobbyDetails")]
    GetLobbyDetails,
    #[serde(rename = "createLobby")]
    CreateLobby,
    #[serde(rename = "joinLobby")]
    JoinLobby,
    #[serde(rename = "quitLobby")]
    QuitLobby,
    #[serde(rename = "listGames")]
    ListGames,
    #[serde(rename = "getGameDetails")]
    GetGameDetails,
    #[serde(rename = "gameMove")]
    GameMove,
    #[serde(rename = "quitGame")]
    QuitGame,
}

#[derive(Deserialize)]
pub(crate) struct WebSocketPayload {
    pub(crate) action: WebSocketAction,
    #[serde(default)]
    pub(crate) data: String,
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
pub(crate) struct GameMovePayload {
    pub(crate) id: String,
    #[serde(rename = "gamePayload")]
    pub(crate) game_payload: String,
}

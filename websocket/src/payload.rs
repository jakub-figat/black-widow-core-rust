use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
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
    #[serde(rename = "gameMove")]
    GameMove(GameMovePayload),
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
pub(crate) struct GameMovePayload {
    pub(crate) id: String,
    #[serde(rename = "gamePayload")]
    pub(crate) game_payload: String,
}

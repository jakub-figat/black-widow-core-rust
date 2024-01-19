use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct WebSocketPayload {
    pub(crate) action: WebSocketAction,
    #[serde(default)]
    payload: String
}

#[derive(Deserialize)]
pub(crate) enum WebSocketAction {
    #[serde(rename = "startGame")]
    StartGame,
    #[serde(rename = "joinGame")]
    JoinGame,
    #[serde(rename = "gameMove")]
    GameMove
}

#[derive(Serialize)]
pub(crate) struct WebSocketResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: WebsocketResponseType,
    payload: String
}

impl WebSocketResponse {
    pub(crate) fn json_from_error(text: &str) -> String {
        let response = WebSocketResponse {
            response_type: WebsocketResponseType::Error,
            payload: text.to_string()
        };

        serde_json::to_string(&response).unwrap()
    }
}

#[derive(Serialize)]
pub(crate) enum WebsocketResponseType {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error
}
use crate::helper::{get_obfuscated_exchange_cards, get_obfuscated_player_cards};
use crate::lobby::Lobby;
use game::step::GameStep;
use game::{Card, Game, GameSettings};
use game::{CardExchange, RoundFinished, RoundInProgress};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Serialize)]
pub(crate) enum ResponseType {
    #[serde(rename = "lobbyList")]
    LobbyList,
    #[serde(rename = "lobbyDetails")]
    LobbyDetails,
    #[serde(rename = "LobbyDeleted")]
    LobbyDeleted,
    #[serde(rename = "gameList")]
    GameList,
    #[serde(rename = "gameDetails")]
    GameDetails,
    #[serde(rename = "gameDeleted")]
    GameDeleted,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize)]
pub(crate) struct LobbyListResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: ResponseType,
    pub(crate) lobbies: HashMap<String, Lobby>,
}

impl LobbyListResponse {
    pub(crate) fn new(lobbies: &HashMap<String, Lobby>) -> LobbyListResponse {
        LobbyListResponse {
            response_type: ResponseType::LobbyList,
            lobbies: lobbies.clone(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct LobbyDetailsResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: ResponseType,
    pub(crate) lobby: Lobby,
}

impl LobbyDetailsResponse {
    pub(crate) fn new(lobby: &Lobby) -> LobbyDetailsResponse {
        LobbyDetailsResponse {
            response_type: ResponseType::LobbyDetails,
            lobby: lobby.clone(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct LobbyDeletedResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: ResponseType,
    pub(crate) id: String,
}

impl LobbyDeletedResponse {
    pub(crate) fn new(id: &str) -> LobbyDeletedResponse {
        LobbyDeletedResponse {
            response_type: ResponseType::LobbyDeleted,
            id: id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct GameListResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: ResponseType,
    games: Vec<ListedGame>,
}

impl GameListResponse {
    pub(crate) fn json_from_game_hashmap(games: &HashMap<String, Game>) -> String {
        let games: Vec<ListedGame> = games
            .iter()
            .map(|(id, game)| ListedGame {
                id: id.clone(),
                players: game.players.to_vec(),
            })
            .collect();

        serde_json::to_string(&GameListResponse {
            response_type: ResponseType::GameList,
            games,
        })
        .unwrap()
    }
}

#[derive(Serialize)]
pub(crate) struct ListedGame {
    pub(crate) id: String,
    pub(crate) players: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct GameDetailsResponse<S: Serialize> {
    #[serde(rename = "responseType")]
    response_type: ResponseType,
    id: String,
    game: ObfuscatedGame<S>,
}

impl<S: Serialize> GameDetailsResponse<S> {
    pub(crate) fn json_from_game(id: &str, game: &Game, player: &str) -> String {
        match game.state.as_ref().unwrap() {
            CardExchange(step) => {
                let state = CardExchangeState {
                    player_exchange_cards: get_obfuscated_exchange_cards(
                        &step.state.cards_to_exchange,
                        player,
                    ),
                    your_exchange_cards: step.state.cards_to_exchange[player].clone(),
                };
                let obfuscated_game = ObfuscatedGame::new(game, &step, state, player);
                GameDetailsResponse::json_from_obfuscated_game_state(id, obfuscated_game)
            }
            RoundInProgress(step) => {
                let state = RoundInProgressState {
                    cards_on_table: step.state.cards_on_table.clone(),
                };
                let obfuscated_game = ObfuscatedGame::new(game, &step, state, player);
                GameDetailsResponse::json_from_obfuscated_game_state(id, obfuscated_game)
            }
            RoundFinished(step) => {
                let state = RoundFinishedState {
                    players_ready: step.state.players_ready.clone(),
                };
                let obfuscated_game = ObfuscatedGame::new(game, &step, state, player);
                GameDetailsResponse::json_from_obfuscated_game_state(id, obfuscated_game)
            }
        }
    }

    fn json_from_obfuscated_game_state(id: &str, game: ObfuscatedGame<S>) -> String {
        let response = GameDetailsResponse {
            response_type: ResponseType::GameDetails,
            id: id.to_string(),
            game,
        };
        serde_json::to_string(&response).unwrap()
    }
}

#[derive(Serialize)]
pub(crate) struct ObfuscatedGame<S: Serialize> {
    settings: GameSettings,
    players: Vec<String>,
    scores: HashMap<String, usize>,
    #[serde(rename = "playerDecks")]
    player_decks: HashMap<String, usize>,
    #[serde(rename = "yourCards")]
    your_cards: HashSet<Card>,
    state: S,
}

impl<S: Serialize> ObfuscatedGame<S> {
    pub(crate) fn new<T>(
        game: &Game,
        step: &GameStep<T>,
        state: S,
        player: &str,
    ) -> ObfuscatedGame<S> {
        ObfuscatedGame {
            settings: game.settings.clone(),
            players: game.players.to_vec(),
            scores: step.scores.clone(),
            player_decks: get_obfuscated_player_cards(&step.player_decks, player),
            your_cards: step.player_decks[player].clone(),
            state,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct CardExchangeState {
    #[serde(rename = "playerExchangeCards")]
    player_exchange_cards: HashMap<String, bool>,
    #[serde(rename = "yourExchangeCards")]
    your_exchange_cards: HashSet<Card>,
}

#[derive(Serialize)]
pub(crate) struct RoundInProgressState {
    #[serde(rename = "cardsOnTable")]
    cards_on_table: HashMap<String, Card>,
}

#[derive(Serialize)]
pub(crate) struct RoundFinishedState {
    #[serde(rename = "playersReady")]
    players_ready: HashMap<String, bool>,
}

#[derive(Serialize)]
pub(crate) struct GameDeletedResponse {
    #[serde(rename = "responseType")]
    response_type: ResponseType,
    id: String,
}

impl GameDeletedResponse {
    pub(crate) fn new(id: &str) -> GameDeletedResponse {
        GameDeletedResponse {
            response_type: ResponseType::LobbyDeleted,
            id: id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct InfoResponse {
    #[serde(rename = "responseType")]
    response_type: ResponseType,
    detail: String,
}

impl InfoResponse {
    pub(crate) fn json_from_detail(text: &str) -> String {
        let response = InfoResponse {
            response_type: ResponseType::Info,
            detail: text.to_string(),
        };

        serde_json::to_string(&response).unwrap()
    }
}

#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: ResponseType,
    detail: String,
}

impl ErrorResponse {
    pub(crate) fn json_from_detail(text: &str) -> String {
        let response = ErrorResponse {
            response_type: ResponseType::Error,
            detail: text.to_string(),
        };

        serde_json::to_string(&response).unwrap()
    }
}

pub(crate) trait ToJson: Serialize {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ToJson for LobbyListResponse {}
impl ToJson for LobbyDetailsResponse {}
impl ToJson for LobbyDeletedResponse {}
impl ToJson for GameListResponse {}
impl ToJson for GameDeletedResponse {}
impl ToJson for InfoResponse {}
impl ToJson for ErrorResponse {}

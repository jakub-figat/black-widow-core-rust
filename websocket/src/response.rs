use crate::helper::{get_obfuscated_exchange_cards, get_obfuscated_player_cards};
use crate::lobby::Lobby;
use crate::response::WebSocketResponse::{
    GameDetailsCardExchange, GameDetailsRoundFinished, GameDetailsRoundInProgress,
};
use game::step::GameStep;
use game::{self, Card, CardExchange, Game, GameSettings, RoundFinished, RoundInProgress};
use serde::Serialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum WebSocketResponse {
    #[serde(rename = "lobbyList")]
    LobbyList(LobbyListResponse),
    #[serde(rename = "lobbyDetails")]
    LobbyDetails(LobbyDetailsResponse),
    #[serde(rename = "LobbyDeleted")]
    LobbyDeleted(IdResponse),
    #[serde(rename = "gameList")]
    GameList(GameListResponse),
    #[serde(rename = "gameDetailsCardExchange")]
    GameDetailsCardExchange(GameDetailsResponse<CardExchangeState>),
    #[serde(rename = "gameDetailsRoundInProgress")]
    GameDetailsRoundInProgress(GameDetailsResponse<RoundInProgressState>),
    #[serde(rename = "gameDetailsCardExchange")]
    GameDetailsRoundFinished(GameDetailsResponse<RoundFinishedState>),
    #[serde(rename = "gameDeleted")]
    GameDeleted(IdResponse),
    #[serde(rename = "error")]
    Error(String),
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct IdResponse {
    pub id: Uuid,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct LobbyListResponse {
    pub lobbies: HashMap<Uuid, Lobby>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct LobbyDetailsResponse {
    pub lobby: Lobby,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct GameListResponse {
    pub games: Vec<ListedGame>,
}

impl GameListResponse {
    pub(crate) fn from_game_hashmap(games: &HashMap<Uuid, Game>) -> GameListResponse {
        let games: Vec<ListedGame> = games
            .iter()
            .map(|(id, game)| ListedGame {
                id: id.clone(),
                players: game.players.to_vec(),
            })
            .collect();

        GameListResponse { games }
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ListedGame {
    pub id: Uuid,
    pub players: Vec<String>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct GameDetailsResponse<S: Serialize> {
    pub id: Uuid,
    pub game: ObfuscatedGame<S>,
}

impl GameDetailsResponse<CardExchangeState> {
    pub(crate) fn new(
        id: &Uuid,
        game: &Game,
        player: &str,
        step: &GameStep<game::CardExchangeState>,
    ) -> GameDetailsResponse<CardExchangeState> {
        let state = CardExchangeState {
            player_exchange_cards: get_obfuscated_exchange_cards(
                &step.state.cards_to_exchange,
                player,
            ),
            your_exchange_cards: step.state.cards_to_exchange[player].clone(),
        };
        let obfuscated_game = ObfuscatedGame::new(game, &step, state, player);
        GameDetailsResponse {
            id: id.clone(),
            game: obfuscated_game,
        }
    }
}

impl GameDetailsResponse<RoundInProgressState> {
    pub(crate) fn new(
        id: &Uuid,
        game: &Game,
        player: &str,
        step: &GameStep<game::RoundInProgressState>,
    ) -> GameDetailsResponse<RoundInProgressState> {
        let state = RoundInProgressState {
            cards_on_table: step.state.cards_on_table.clone(),
        };
        let obfuscated_game = ObfuscatedGame::new(game, &step, state, player);
        GameDetailsResponse {
            id: id.clone(),
            game: obfuscated_game,
        }
    }
}

impl GameDetailsResponse<RoundFinishedState> {
    pub(crate) fn new(
        id: &Uuid,
        game: &Game,
        player: &str,
        step: &GameStep<game::RoundFinishedState>,
    ) -> GameDetailsResponse<RoundFinishedState> {
        let state = RoundFinishedState {
            players_ready: step.state.players_ready.clone(),
        };
        let obfuscated_game = ObfuscatedGame::new(game, &step, state, player);
        GameDetailsResponse {
            id: id.clone(),
            game: obfuscated_game,
        }
    }
}

pub(crate) fn get_obfuscated_game_details_json(id: &Uuid, game: &Game, player: &str) -> String {
    match &game.state {
        CardExchange(step) => GameDetailsCardExchange(
            GameDetailsResponse::<CardExchangeState>::new(id, &game, player, &step),
        )
        .to_json(),
        RoundInProgress(step) => {
            GameDetailsRoundInProgress(GameDetailsResponse::<RoundInProgressState>::new(
                id, &game, player, &step,
            ))
            .to_json()
        }
        RoundFinished(step) => GameDetailsRoundFinished(
            GameDetailsResponse::<RoundFinishedState>::new(id, &game, player, &step),
        )
        .to_json(),
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ObfuscatedGame<S: Serialize> {
    pub settings: GameSettings,
    pub players: Vec<String>,
    pub scores: HashMap<String, usize>,
    #[serde(rename = "isFinished")]
    pub is_finished: bool,
    #[serde(rename = "playerDecks")]
    pub player_decks: HashMap<String, usize>,
    #[serde(rename = "yourCards")]
    pub your_cards: HashSet<Card>,
    pub state: S,
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
            is_finished: game.is_finished,
            player_decks: get_obfuscated_player_cards(&step.player_decks, player),
            your_cards: step.player_decks[player].clone(),
            state,
        }
    }
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct CardExchangeState {
    #[serde(rename = "playerExchangeCards")]
    pub player_exchange_cards: HashMap<String, bool>,
    #[serde(rename = "yourExchangeCards")]
    pub your_exchange_cards: HashSet<Card>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct RoundInProgressState {
    #[serde(rename = "cardsOnTable")]
    pub cards_on_table: HashMap<String, Card>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct RoundFinishedState {
    #[serde(rename = "playersReady")]
    pub players_ready: HashMap<String, bool>,
}

impl WebSocketResponse {
    pub(crate) fn to_json(&self) -> String {
        match self {
            WebSocketResponse::Error(text) => json!({"error": text.clone()}).to_string(),
            _ => serde_json::to_string(self).unwrap(),
        }
    }
}

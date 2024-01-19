use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use serde::Serialize;
use game::{Card, Game, GameSettings};
use game::game::GameState::{CardExchange, RoundInProgress, RoundFinished};
use crate::helper::{get_obfuscated_exchange_cards, get_obfuscated_player_cards};


#[derive(Serialize)]
pub(crate) enum ResponseType {
    #[serde(rename = "gameList")]
    GameList,
    #[serde(rename = "gameDetails")]
    GameDetails,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error
}


#[derive(Serialize)]
pub(crate) struct GameListResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: ResponseType,
    games: Vec<ListedGame>
}

impl GameListResponse {
    pub(crate) fn json_from_game_hashmap(games: &HashMap<String, Game>) -> String {
        let games: Vec<ListedGame> = games.iter()
            .map(|(id, game)| ListedGame {
                id: id.clone(),
                players: game.players.to_vec()
            })
            .collect();

        serde_json::to_string(
            &GameListResponse {
                response_type: ResponseType::GameList,
                games
            }
        ).unwrap()
    }
}

#[derive(Serialize)]
pub(crate) struct ListedGame {
    pub(crate) id: String,
    pub(crate) players: Vec<String>
}


// TODO: IMPORTANT to obfuscate state and check if player belongs to given game
#[derive(Serialize)]
pub(crate) struct GameDetails<S: Serialize> {
    id: String,
    game: ObfuscatedGame<S>
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
    state: S
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

impl<S: Serialize> GameDetails<S> {
    // TODO: boilerplate af, needs refactor
    pub(crate) fn json_from_game(game: &Game, player: &str) -> String {
        let settings = game.settings.clone();
        let players = game.players.clone();
        match game.state.as_ref().unwrap() {
            CardExchange(step) => {
                let obfuscated_game = ObfuscatedGame {
                    settings,
                    players,
                    scores: step.scores.clone(),
                    player_decks: get_obfuscated_player_cards(
                        &step.player_decks,
                        player
                    ),
                    your_cards: step.player_decks[player].clone(),
                    state: CardExchangeState {
                        player_exchange_cards: get_obfuscated_exchange_cards(
                            &step.state.cards_to_exchange,
                            player
                        ),
                        your_exchange_cards: step.state.cards_to_exchange[player].clone(),
                    }
                };
                serde_json::to_string(&obfuscated_game).unwrap()
            },
            RoundInProgress(step) => {
                let obfuscated_game = ObfuscatedGame {
                    settings,
                    players,
                    scores: step.scores.clone(),
                    player_decks: get_obfuscated_player_cards(
                        &step.player_decks,
                        player
                    ),
                    your_cards: step.player_decks[player].clone(),
                    state: RoundInProgressState {
                        cards_on_table: step.state.cards_on_table.clone()
                    }
                };
                serde_json::to_string(&obfuscated_game).unwrap()
            },
            RoundFinished(step) => {
                let obfuscated_game = ObfuscatedGame {
                    settings,
                    players,
                    scores: step.scores.clone(),
                    player_decks: get_obfuscated_player_cards(
                        &step.player_decks,
                        player
                    ),
                    your_cards: step.player_decks[player].clone(),
                    state: RoundFinishedState {
                        players_ready: step.state.players_ready.clone()
                    }
                };
                serde_json::to_string(&obfuscated_game).unwrap()
            }
        }
    }
}


#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    #[serde(rename = "responseType")]
    pub(crate) response_type: ResponseType,
    detail: String
}

impl ErrorResponse {
    pub(crate) fn json_from_detail(text: &str) -> String {
        let response = ErrorResponse {
            response_type: ResponseType::Error,
            detail: text.to_string()
        };

        serde_json::to_string(&response).unwrap()
    }
}
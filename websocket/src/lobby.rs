use serde::Serialize;
use crate::error::ValidationError;

#[derive(Serialize, Clone)]
pub(crate) struct Lobby {
    #[serde(rename = "maxPlayers")]
    max_players: usize,
    players: Vec<String>
}

impl Lobby {
    pub(crate) fn new_by_player(max_players: usize, player: &str) -> Result<Lobby, ValidationError> {
        if max_players < 3 || max_players > 4 {
            Err(ValidationError("Invalid lobby max players".to_string()))?
        }

        Ok(Lobby { max_players, players: vec![player.to_string()] })
    }
}
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS, Clone)]
#[ts(export)]
pub struct Lobby {
    #[serde(rename = "maxPlayers")]
    pub max_players: usize,
    #[serde(rename = "maxScore")]
    pub max_score: usize,
    pub players: Vec<String>,
}

impl Lobby {
    pub(crate) fn new_by_player(
        max_players: usize,
        max_score: usize,
        player: &str,
    ) -> Result<Lobby, String> {
        if max_players < 3 || max_players > 4 {
            Err("Invalid lobby max players".to_string())?
        }

        Ok(Lobby {
            max_players,
            max_score,
            players: vec![player.to_string()],
        })
    }
}

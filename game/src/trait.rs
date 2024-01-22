use crate::error::GameResult;
use crate::GameError;
use serde::Deserialize;

pub trait PayloadHandler<'a, P: Deserialize<'a>> {
    fn validate_payload(&self, payload: &P, player: &str) -> GameResult<()>;
    fn dispatch_payload(&mut self, payload: &P, player: &str);

    fn handle_payload(&mut self, json_payload: &'a str, player: &str) -> Result<(), GameError> {
        match serde_json::from_str(json_payload) {
            Ok(payload) => {
                self.validate_payload(&payload, player)?;
                self.dispatch_payload(&payload, player);
                Ok(())
            }
            Err(error) => Err(GameError(error.to_string())),
        }
    }
}

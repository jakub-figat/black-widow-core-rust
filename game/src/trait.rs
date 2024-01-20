use crate::error::GameResult;
use serde::Deserialize;
use std::error::Error;

pub trait PayloadHandler<'a, P: Deserialize<'a>> {
    fn validate_payload(&self, payload: &P, player: &str) -> GameResult<()>;
    fn dispatch_payload(&mut self, payload: &P, player: &str);

    fn handle_payload(
        &mut self,
        json_payload: &'a str,
        player: &str,
    ) -> Result<(), Box<dyn Error>> {
        let payload = serde_json::from_str(json_payload)?;
        self.validate_payload(&payload, player)?;
        self.dispatch_payload(&payload, player);
        Ok(())
    }
}

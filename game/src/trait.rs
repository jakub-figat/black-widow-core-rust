use std::error::Error;
use serde::Deserialize;
use crate::error::{GameResult};

pub trait PayloadHandler<'a, P: Deserialize<'a>> {
    fn validate_payload(&self, payload: &P, player: &str) -> GameResult<()>;
    fn dispatch_payload(&mut self, payload: &P, player: &str);

    fn handle_payload(
        &mut self,
        json_payload: &'a str,
        player: &str,
        error_writer: fn(err: Box<dyn Error>)
    ) {
        if let Ok(payload) = serde_json::from_str(json_payload) {
            self.validate_payload(&payload, player).unwrap_or_else(|error| {
                error_writer(Box::new(error));
            });
            self.dispatch_payload(&payload, player);
        }
    }
}
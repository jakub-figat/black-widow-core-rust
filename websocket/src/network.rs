use crate::response::WebSocketResponse;
use crate::response::WebSocketResponse::GameDetails;
use crate::response::{game_to_json, ToJson};
use crate::WebSocketState;
use axum::extract::ws::Message;
use game::Game;
use std::ops;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

type ControlFlow = ops::ControlFlow<(), ()>;

pub(crate) async fn send_text_or_break(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow {
    if sender.send(Message::Text(text.to_string())).await.is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub(crate) fn broadcast_text_or_break(
    text: &str,
    broadcast_sender: &mut broadcast::Sender<Message>,
) -> ControlFlow {
    if broadcast_sender
        .send(Message::Text(text.to_string()))
        .is_err()
    {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub(crate) async fn broadcast_text_to_players_or_break(
    text: &str,
    players: &[String],
    state: Arc<WebSocketState>,
) -> ControlFlow {
    let player_connections = state.player_connections.read().await;
    for player in players {
        let mut sender = player_connections.get(player).unwrap().clone();
        send_text_or_break(text, &mut sender).await?
    }
    ControlFlow::Continue(())
}

pub(crate) async fn broadcast_game_to_players_or_break(
    id: &str,
    game: &Game,
    state: Arc<WebSocketState>,
) -> ControlFlow {
    let player_connections = state.player_connections.read().await;
    for player in &game.players {
        let mut sender = player_connections.get(player).unwrap().clone();
        send_text_or_break(
            &GameDetails(game_to_json(id, game, player)).to_json(),
            &mut sender,
        )
        .await?
    }
    ControlFlow::Continue(())
}

pub(crate) async fn send_error_or_break(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow {
    send_text_or_break(
        &WebSocketResponse::Error(text.to_string()).to_json(),
        sender,
    )
    .await
}

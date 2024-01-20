use crate::game_action::{
    create_lobby, game_move, get_game_details, get_lobby_details, join_lobby, list_games,
    list_lobbies, quit_game, quit_lobby,
};
use crate::helper::{parse_uuid, send_error_or_break, send_text_or_break};
use crate::payload::{WebSocketAction::*, WebSocketPayload};
use crate::WebSocketGameState;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use std::ops::ControlFlow;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

pub(crate) async fn handle(
    websocket: WebSocketUpgrade,
    State(state): State<Arc<WebSocketGameState>>,
) -> impl IntoResponse {
    // TODO: implement decoding JWT here
    let user = "1".to_string();
    websocket.on_upgrade(move |socket| handle_websocket(user, socket, state))
}

pub(crate) async fn handle_websocket(
    user: String,
    websocket: WebSocket,
    state: Arc<WebSocketGameState>,
) {
    let (mut sink, mut stream) = websocket.split();
    let (mut sender, mut receiver) = mpsc::channel(128);

    let sender2 = sender.clone();
    let mut broadcast_subscriber = state.broadcast_sender.subscribe();

    // wrapped websocket sender
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if sink.send(message).await.is_err() {
                break;
            }
        }
    });

    // task for handling messages received from websocket
    let mut receiver_task = tokio::spawn(async move {
        while let Some(Ok(message)) = stream.next().await {
            if handle_message(message, user.as_str(), state.clone(), &mut sender)
                .await
                .is_break()
            {
                break;
            }
        }
    });

    let mut broadcast_receiver_task = tokio::spawn(async move {
        while let Ok(message) = broadcast_subscriber.recv().await {
            if sender2.send(message).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = &mut receiver_task => broadcast_receiver_task.abort(),
        _ = &mut broadcast_receiver_task => receiver_task.abort()
    }
}

pub(crate) async fn handle_message(
    message: Message,
    player: &str,
    state: Arc<WebSocketGameState>,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow<(), ()> {
    let mut broadcast_sender = state.broadcast_sender.clone();

    return match message {
        Message::Text(text) => {
            handle_text_message(text, player, sender, &mut broadcast_sender, state).await
        }
        Message::Close(_) => ControlFlow::Break(()),
        _ => send_text_or_break("Invalid message", sender).await,
    };
}

pub(crate) async fn handle_text_message(
    text: String,
    player: &str,
    sender: &mut mpsc::Sender<Message>,
    broadcast_sender: &mut broadcast::Sender<Message>,
    state: Arc<WebSocketGameState>,
) -> ControlFlow<(), ()> {
    // TODO: reduce duplication of uuid parsing with macro?
    match serde_json::from_str::<WebSocketPayload>(&text) {
        Ok(payload) => match payload.action {
            ListLobbies => list_lobbies(sender, state).await,
            GetLobbyDetails => match parse_uuid(&payload.data) {
                Ok(id) => get_lobby_details(&id, sender, state).await,
                Err(_) => send_error_or_break("Invalid UUID", sender).await,
            },
            CreateLobby => create_lobby(player, broadcast_sender, state).await,
            JoinLobby => match parse_uuid(&payload.data) {
                Ok(id) => join_lobby(&id, player, sender, broadcast_sender, state).await,
                Err(_) => send_error_or_break("Invalid UUID", sender).await,
            },
            QuitLobby => match parse_uuid(&payload.data) {
                Ok(id) => quit_lobby(&id, player, sender, broadcast_sender, state).await,
                Err(_) => send_error_or_break("Invalid UUID", sender).await,
            },
            ListGames => list_games(sender, state).await,
            GetGameDetails => match parse_uuid(&payload.data) {
                Ok(id) => get_game_details(&id, player, sender, state).await,
                Err(_) => send_error_or_break("Invalid UUID", sender).await,
            },
            GameMove => game_move(&payload.data, player, sender, broadcast_sender, state).await,
            QuitGame => match parse_uuid(&payload.data) {
                Ok(id) => quit_game(&id, player, sender, broadcast_sender, state).await,
                Err(_) => send_error_or_break("Invalid UUID", sender).await,
            },
        },
        Err(_) => send_error_or_break("Invalid JSON payload", sender).await,
    }
}

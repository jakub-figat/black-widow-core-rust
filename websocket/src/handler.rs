use crate::game_action::{
    create_lobby, game_move, get_game_details, get_lobby_details, join_lobby, list_games,
    list_lobbies, quit_game, quit_lobby,
};
use crate::payload::{WebSocketAction::*, WebSocketPayload};
use crate::response::ErrorResponse;
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
    websocket.on_upgrade(move |socket| handle_websocket(socket, state))
}

pub(crate) async fn handle_websocket(websocket: WebSocket, state: Arc<WebSocketGameState>) {
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
            if handle_message(message, state.clone(), &mut sender)
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
    state: Arc<WebSocketGameState>,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow<(), ()> {
    let mut broadcast_sender = state.broadcast_sender.clone();

    return match message {
        Message::Text(text) => {
            handle_text_message(text, sender, &mut broadcast_sender, state).await
        }
        Message::Close(_) => ControlFlow::Break(()),
        _ => send_text_or_break("Invalid message", sender).await,
    };
}

pub(crate) async fn handle_text_message(
    text: String,
    sender: &mut mpsc::Sender<Message>,
    broadcast_sender: &mut broadcast::Sender<Message>,
    state: Arc<WebSocketGameState>,
) -> ControlFlow<(), ()> {
    match serde_json::from_str::<WebSocketPayload>(&text) {
        Ok(payload) => match payload.action {
            ListLobbies => list_lobbies(sender, state).await,
            GetLobbyDetails => get_lobby_details("1", sender, state).await,
            CreateLobby => create_lobby("1", broadcast_sender, state).await,
            JoinLobby => join_lobby("1", "1", sender, broadcast_sender, state).await,
            QuitLobby => quit_lobby("1", "1", sender, broadcast_sender, state).await,
            ListGames => list_games(sender, state).await,
            GetGameDetails => get_game_details("1", "2", sender, state).await,
            GameMove => game_move("1", "1", "1", sender, broadcast_sender, state).await,
            QuitGame => quit_game("1", "1", sender, broadcast_sender, state).await,
        },
        Err(_) => {
            send_text_or_break(
                &ErrorResponse::json_from_detail("Invalid JSON payload"),
                sender,
            )
            .await
        }
    }
}

pub(crate) async fn send_text_or_break(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow<(), ()> {
    if sender.send(Message::Text(text.to_string())).await.is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub(crate) fn broadcast_text_or_break(
    text: &str,
    sender: &mut broadcast::Sender<Message>,
) -> ControlFlow<(), ()> {
    if sender.send(Message::Text(text.to_string())).is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

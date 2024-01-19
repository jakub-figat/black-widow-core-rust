use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::Arc;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};
use crate::game_action::{list_games, start_game};
use crate::payload::{WebSocketAction, WebSocketPayload};
use crate::response::ErrorResponse;
use crate::WebSocketGameState;

pub(crate) async fn handle(
    websocket: WebSocketUpgrade,
    State(state): State<Arc<WebSocketGameState>>,
    ConnectInfo(address): ConnectInfo<SocketAddr>
) -> impl IntoResponse {
    websocket.on_upgrade(move |socket| handle_websocket(socket, address, state))
}


pub(crate) async fn handle_websocket(websocket: WebSocket, address: SocketAddr, state: Arc<WebSocketGameState>) {
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
            if handle_message(message, state.clone(), &mut sender, &address).await.is_break() {
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
    address: &SocketAddr
) -> ControlFlow<(), ()> {
    let mut broadcast_sender = state.broadcast_sender.clone();

    return match message {
        Message::Text(text) => {
            handle_text_message(text, sender, &mut broadcast_sender, state, address).await
        }
        Message::Close(_) => {
            ControlFlow::Break(())
        }
        _ => {
            send_text_or_break("Invalid message", sender).await
        }
    }
}


pub(crate) async fn handle_text_message(
    text: String,
    sender: &mut mpsc::Sender<Message>,
    broadcast_sender: &mut broadcast::Sender<Message>,
    state: Arc<WebSocketGameState>,
    address: &SocketAddr
) -> ControlFlow<(), ()> {
    match serde_json::from_str::<WebSocketPayload>(&text) {
        Ok(payload) => {
            match payload.action {
                WebSocketAction::StartGame => {
                    start_game(broadcast_sender, state, address).await
                }
                WebSocketAction::ListGames => {
                    list_games(sender, state).await
                }
                _ => {
                    send_text_or_break(
                        &ErrorResponse::json_from_detail("Invalid action"),
                        sender
                    ).await // TODO: this will be removed when all actions are handled
                }
            }
        }
        Err(_) => {
            send_text_or_break(
                &ErrorResponse::json_from_detail("Invalid JSON payload"),
                sender
            ).await
        }
    }
}

pub(crate) async fn send_text_or_break(text: &str, sender: &mut mpsc::Sender<Message>) -> ControlFlow<(), ()> {
    if sender.send(Message::Text(text.to_string())).await.is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub(crate) fn broadcast_text_or_break(text: &str, sender: &mut broadcast::Sender<Message>) -> ControlFlow<(), ()> {
    if sender.send(Message::Text(text.to_string())).is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}
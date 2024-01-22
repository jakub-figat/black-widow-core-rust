use crate::game_action::{
    create_lobby, game_move, get_game_details, get_lobby_details, join_lobby, list_games,
    list_lobbies, quit_game, quit_lobby,
};
use crate::helper::parse_uuid_from_payload;
use crate::network::{send_error_or_break, send_text_or_break};
use crate::payload::{WebSocketAction::*, WebSocketPayload};
use crate::WebSocketState;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::ops::ControlFlow;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

pub(crate) async fn handle(
    websocket: WebSocketUpgrade,
    State(state): State<Arc<WebSocketState>>,
) -> impl IntoResponse {
    // TODO: implement decoding JWT here
    let user = Uuid::new_v4().to_string();
    websocket.on_upgrade(move |socket| handle_websocket(socket, user, state))
}

pub(crate) async fn handle_websocket(
    websocket: WebSocket,
    user: String,
    state: Arc<WebSocketState>,
) {
    let (sink, stream) = websocket.split();
    let (mut sender, receiver) = mpsc::channel(128);
    tokio::spawn(wrap_sink(sink, receiver));

    if let Err(text) = add_player_to_connections(user.as_str(), sender.clone(), state.clone()).await
    {
        send_error_or_break(&text, &mut sender).await;
        return;
    }

    let mut receiver_task = tokio::spawn(read(stream, sender.clone(), user.clone(), state.clone()));
    let mut broadcast_receiver_task = tokio::spawn(read_broadcast(
        sender.clone(),
        state.broadcast_sender.subscribe(),
    ));

    tokio::select! {
        _ = &mut receiver_task => broadcast_receiver_task.abort(),
        _ = &mut broadcast_receiver_task => receiver_task.abort()
    }
    // TODO: test reconnecting
    let mut player_connections = state.player_connections.write().await;
    player_connections.remove(&user);
}

async fn add_player_to_connections(
    player: &str,
    sender: mpsc::Sender<Message>,
    state: Arc<WebSocketState>,
) -> Result<(), String> {
    let mut player_connections = state.player_connections.write().await;
    if player_connections.contains_key(player) {
        Err("This player is already connected")?
    }
    player_connections.insert(player.to_string(), sender);
    Ok(())
}

async fn read(
    mut stream: SplitStream<WebSocket>,
    mut sender: mpsc::Sender<Message>,
    user: String,
    state: Arc<WebSocketState>,
) {
    while let Some(Ok(message)) = stream.next().await {
        if handle_message(message, &user, state.clone(), &mut sender)
            .await
            .is_break()
        {
            break;
        }
    }
}

async fn read_broadcast(
    sender: mpsc::Sender<Message>,
    mut broadcast_receiver: broadcast::Receiver<Message>,
) {
    while let Ok(message) = broadcast_receiver.recv().await {
        if sender.send(message).await.is_err() {
            break;
        }
    }
}

async fn wrap_sink(mut sink: SplitSink<WebSocket, Message>, mut receiver: mpsc::Receiver<Message>) {
    while let Some(message) = receiver.recv().await {
        if sink.send(message).await.is_err() {
            break;
        }
    }
}

pub(crate) async fn handle_message(
    message: Message,
    player: &str,
    state: Arc<WebSocketState>,
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
    state: Arc<WebSocketState>,
) -> ControlFlow<(), ()> {
    // TODO: reduce duplication of uuid parsing with macro?
    match serde_json::from_str::<WebSocketPayload>(&text) {
        Ok(payload) => match payload.action {
            ListLobbies => list_lobbies(sender, state).await,
            GetLobbyDetails => match parse_uuid_from_payload(&payload.data) {
                Ok(id) => get_lobby_details(&id, sender, state).await,
                Err(error) => send_error_or_break(&error.to_string(), sender).await,
            },
            CreateLobby => {
                create_lobby(&payload.data, player, sender, broadcast_sender, state).await
            }
            JoinLobby => match parse_uuid_from_payload(&payload.data) {
                Ok(id) => join_lobby(&id, player, sender, broadcast_sender, state).await,
                Err(error) => send_error_or_break(&error.to_string(), sender).await,
            },
            QuitLobby => match parse_uuid_from_payload(&payload.data) {
                Ok(id) => quit_lobby(&id, player, sender, broadcast_sender, state).await,
                Err(error) => send_error_or_break(&error.to_string(), sender).await,
            },
            ListGames => list_games(sender, state).await,
            GetGameDetails => match parse_uuid_from_payload(&payload.data) {
                Ok(id) => get_game_details(&id, player, sender, state).await,
                Err(error) => send_error_or_break(&error.to_string(), sender).await,
            },
            GameMove => game_move(&payload.data, player.to_string(), sender, state).await,
            QuitGame => match parse_uuid_from_payload(&payload.data) {
                Ok(id) => quit_game(&id, player.to_string(), sender, broadcast_sender, state).await,
                Err(error) => send_error_or_break(&error.to_string(), sender).await,
            },
        },
        Err(_) => send_error_or_break("Invalid JSON payload", sender).await,
    }
}

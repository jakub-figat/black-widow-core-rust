use crate::game_action::{
    create_lobby, game_move, get_game_details, get_lobby_details, join_lobby, list_games,
    list_lobbies, quit_game, quit_lobby,
};
use crate::helper::parse_uuid_from_payload;
use crate::network::{send_error_or_break, send_text_or_break};
use crate::payload::{WebSocketAction::*, WebSocketPayload};
use crate::WebSocketState;
use axum::body::Body;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};
use std::ops::ControlFlow;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

pub(crate) async fn handle(
    websocket: WebSocketUpgrade,
    State(state): State<Arc<WebSocketState>>,
) -> impl IntoResponse {
    // TODO: implement decoding JWT here
    let user = "1".to_string();

    {
        let player_connections = state.player_connections.read().await;
        if player_connections.contains_key(&user) {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("This player is already connected"))
                .unwrap();
        }
    }

    websocket.on_upgrade(move |socket| handle_websocket(user, socket, state))
}

pub(crate) async fn handle_websocket(
    user: String,
    websocket: WebSocket,
    state: Arc<WebSocketState>,
) {
    let (mut sink, mut stream) = websocket.split();
    let (sender, mut receiver) = mpsc::channel(128);

    {
        let mut player_connections = state.player_connections.write().await;
        player_connections.insert(user.clone(), sender.clone());
    }

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
    let state2 = state.clone();
    let user2 = user.clone();
    let mut sender3 = sender.clone();
    let mut receiver_task = tokio::spawn(async move {
        while let Some(Ok(message)) = stream.next().await {
            if handle_message(message, user2.as_str(), state2.clone(), &mut sender3)
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

    let mut player_connections = state.player_connections.write().await;
    player_connections.remove(&user);
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
            GameMove => game_move(&payload.data, player, sender, broadcast_sender, state).await,
            QuitGame => match parse_uuid_from_payload(&payload.data) {
                Ok(id) => quit_game(&id, player, sender, broadcast_sender, state).await,
                Err(error) => send_error_or_break(&error.to_string(), sender).await,
            },
        },
        Err(_) => send_error_or_break("Invalid JSON payload", sender).await,
    }
}

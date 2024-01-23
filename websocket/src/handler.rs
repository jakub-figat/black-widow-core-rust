use crate::error::HandlerError::{ActionError, SenderError};
use crate::game_action::{
    card_exchange_move, claim_readiness_move, create_lobby, get_game_details, get_lobby_details,
    join_lobby, list_games, list_lobbies, place_card_move, quit_game, quit_lobby,
};
use crate::network::send_error;
use crate::payload::{WebSocketPayload, WebSocketPayload::*};
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
        let _ = send_error(&text, &mut sender).await;
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

    match message {
        Message::Text(text) => {
            match handle_text_message(text, player, sender, &mut broadcast_sender, state).await {
                Ok(_) => ControlFlow::Continue(()),
                Err(text) => {
                    println!("{}", text);
                    ControlFlow::Break(())
                }
            }
        }
        Message::Close(_) => ControlFlow::Break(()),
        _ => match sender
            .send(Message::Text("Invalid message type".to_string()))
            .await
        {
            Ok(_) => ControlFlow::Continue(()),
            Err(error) => {
                println!("{}", error.to_string());
                ControlFlow::Break(())
            }
        },
    }
}

pub(crate) async fn handle_text_message(
    text: String,
    player: &str,
    sender: &mut mpsc::Sender<Message>,
    broadcast_sender: &mut broadcast::Sender<Message>,
    state: Arc<WebSocketState>,
) -> Result<(), String> {
    let payload_result = serde_json::from_str::<WebSocketPayload>(&text);
    if let Err(error) = payload_result {
        send_error(&error.to_string(), sender).await?;
        return Ok(());
    };

    let handler_result = match payload_result.unwrap() {
        ListLobbies => list_lobbies(sender, state).await,
        GetLobbyDetails(payload) => get_lobby_details(&payload.id, sender, state).await,
        CreateLobby(create_lobby_payload) => {
            create_lobby(&create_lobby_payload, player, broadcast_sender, state).await
        }
        JoinLobby(payload) => join_lobby(&payload.id, player, broadcast_sender, state).await,
        QuitLobby(payload) => quit_lobby(&payload.id, player, broadcast_sender, state).await,
        ListGames => list_games(sender, state).await,
        GetGameDetails(payload) => {
            get_game_details(&payload.id, player.to_string(), sender, state).await
        }
        CardExchangeMove(payload) => card_exchange_move(&payload, player.to_string(), state).await,
        PlaceCardMove(payload) => place_card_move(&payload, player.to_string(), state).await,
        ClaimReadinessMove(payload) => {
            claim_readiness_move(&payload, player.to_string(), state).await
        }
        QuitGame(payload) => {
            quit_game(&payload.id, player.to_string(), broadcast_sender, state).await
        }
    };

    if let Err(error) = handler_result {
        match error {
            ActionError(text) => send_error(&text, sender).await?,
            SenderError(error) => Err(error.to_string())?,
        }
    };
    Ok(())
}

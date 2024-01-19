use std::collections::HashMap;
use std::net::SocketAddr;
use std::ops::ControlFlow;
use std::sync::{Arc, Mutex};
use axum::extract::{ConnectInfo, State};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Router};
use axum::routing::get;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};
use game::Game;
use serde::Deserialize;


pub async fn start_game_server() {
    let state = Arc::new(WebSocketGameState::new());
    let app = Router::new()
        .route("/ws", get(handle))
        .with_state(state);

    println!("starting...");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}


struct WebSocketGameState {
    games: Arc<Mutex<HashMap<String, Game>>>,
    broadcast_sender: broadcast::Sender<Message>
}

impl WebSocketGameState {
    pub fn new() -> WebSocketGameState {
        WebSocketGameState {
            games: Arc::new(Mutex::new(HashMap::new())),
            broadcast_sender: broadcast::channel::<Message>(128).0
        }
    }
}


async fn handle(
    websocket: WebSocketUpgrade,
    State(state): State<Arc<WebSocketGameState>>,
    ConnectInfo(address): ConnectInfo<SocketAddr>
) -> impl IntoResponse {
    websocket.on_upgrade(move |socket| handle_socket(socket, address, state))
}

async fn handle_socket(websocket: WebSocket, address: SocketAddr, state: Arc<WebSocketGameState>) {
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

async fn handle_message(
    message: Message,
    state: Arc<WebSocketGameState>,
    sender: &mut mpsc::Sender<Message>,
    address: &SocketAddr
) -> ControlFlow<(), ()> {
    let mut broadcast_sender = state.broadcast_sender.clone();

    return match message {
        Message::Text(text) => {
            handle_text_message(text, sender, &mut broadcast_sender, address).await
        }
        Message::Close(_) => {
            ControlFlow::Break(())
        }
        _ => {
            send_text_or_break("Invalid message", sender).await
        }
    }
}

async fn handle_text_message(
    text: String,
    sender: &mut mpsc::Sender<Message>,
    broadcast_sender: &mut broadcast::Sender<Message>,
    address: &SocketAddr
) -> ControlFlow<(), ()> {
    match serde_json::from_str::<WebSocketPayload>(&text) {
        Ok(payload) => {
            match payload.action {
                WebSocketAction::StartGame => {
                    broadcast_text_or_break(
                        &format!("{} wants to start a game", address),
                        broadcast_sender,
                    )
                }
                _ => {
                    send_text_or_break("Invalid action", sender).await
                }
            }
        }
        Err(error) => {
            send_text_or_break(
                &format!("Invalid JSON payload: {}", error.to_string()),
                sender
            ).await
        }
    }
}

async fn send_text_or_break(text: &str, sender: &mut mpsc::Sender<Message>) -> ControlFlow<(), ()> {
    if sender.send(Message::Text(text.to_string())).await.is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

fn broadcast_text_or_break(text: &str, sender: &mut broadcast::Sender<Message>) -> ControlFlow<(), ()> {
    if sender.send(Message::Text(text.to_string())).is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

#[derive(Deserialize)]
struct WebSocketPayload {
    action: WebSocketAction
}

#[derive(Deserialize)]
enum WebSocketAction {
    #[serde(rename = "startGame")]
    StartGame,
    #[serde(rename = "joinGame")]
    JoinGame,
    #[serde(rename = "gameMove")]
    GameMove
}

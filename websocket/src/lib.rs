use std::collections::HashMap;
use std::net::SocketAddr;
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


pub struct WebSocketGameState {
    games: Arc<Mutex<HashMap<String, Game>>>,
    broadcast_sender: broadcast::Sender<Message>
}

impl WebSocketGameState {
    pub fn new() -> WebSocketGameState {
        broadcast::channel::<Message>(128).0;

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
            handle_player_message(message, state.clone(), &mut sender, &address).await
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

async fn handle_player_message(
    message: Message,
    state: Arc<WebSocketGameState>,
    sender: &mut mpsc::Sender<Message>,
    address: &SocketAddr
) {

    let broadcast_sender = state.broadcast_sender.clone();
    match message {
        Message::Text(text) => {
            if text == "start game" {
                broadcast_sender.send(Message::Text(format!("{} wants to start a game", address))).unwrap();
            } else {
                sender.send(Message::Text("unknown command".to_string())).await.unwrap();
            }
        }
        _ => {
            sender.send(Message::Text("unknown command".to_string())).await.unwrap();
        }
    }
}

// dispatch player message
// improve horrible error handling

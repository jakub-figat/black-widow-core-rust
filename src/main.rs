#[tokio::main]
async fn main() {
    websocket::start_game_server().await;
}

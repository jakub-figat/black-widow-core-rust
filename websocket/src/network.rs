use crate::response::ErrorResponse;
use axum::extract::ws::Message;
use std::ops;
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
    sender: &mut broadcast::Sender<Message>,
) -> ControlFlow {
    if sender.send(Message::Text(text.to_string())).is_err() {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

pub(crate) async fn send_error_or_break(
    text: &str,
    sender: &mut mpsc::Sender<Message>,
) -> ControlFlow {
    crate::helper::send_text_or_break(&ErrorResponse::json_from_detail(text), sender).await
}

use crate::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    let bus = state.bus.clone();
    ws.on_upgrade(move |socket| run(socket, bus))
}

async fn run(socket: WebSocket, bus: Arc<crate::events::EventBus>) {
    let (mut tx, mut rx) = socket.split();
    let mut events = bus.subscribe();
    loop {
        tokio::select! {
            event = events.recv() => match event {
                Ok(event) => {
                    let message = serde_json::to_string(&event).unwrap_or_default();
                    if tx.send(Message::Text(message.into())).await.is_err() {
                        break;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(_) => break,
            },
            incoming = rx.next() => match incoming {
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(Message::Ping(v))) => {
                    if tx.send(Message::Pong(v)).await.is_err() {
                        break;
                    }
                }
                Some(Ok(_)) => {}
                Some(Err(_)) => break,
            }
        }
    }
}

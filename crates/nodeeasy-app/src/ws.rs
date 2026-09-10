use crate::events::EventBus;
use axum::{extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State}, response::Response};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;

pub async fn handler(ws: WebSocketUpgrade, State(bus): State<Arc<EventBus>>) -> Response { ws.on_upgrade(move |socket| run(socket, bus)) }

async fn run(socket: WebSocket, bus: Arc<EventBus>) {
    let (mut tx, mut rx) = socket.split();
    let mut events = bus.subscribe();
    loop {
        tokio::select! {
            event = events.recv() => match event {
                Ok(event) => if tx.send(Message::Text(serde_json::to_string(&event).unwrap_or_default().into())).await.is_err() { break },
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(_) => break,
            },
            incoming = rx.next() => match incoming {
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(Message::Ping(v))) => { if tx.send(Message::Pong(v)).await.is_err() { break } },
                Some(Ok(_)) => {},
                Some(Err(_)) => break,
            }
        }
    }
}

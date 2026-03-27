use axum::{
    extract::{
        State,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};

use crate::message::Message;
use crate::state::SharedState;
use futures_util::{sink::SinkExt, stream::StreamExt};

pub async fn handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(socket: WebSocket, state: SharedState) {
    let mut broadcast = state.broadcast.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(Message::Custom(content)) = broadcast.recv().await {
            if sender.send(WsMessage::Text(content.into())).await.is_err() {
                break;
            }
        }
    });

    let _ = state
        .broadcast
        .send(Message::Custom("Clinvas Hub Connected".to_string()));

    println!("Frontend connected!");
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                WsMessage::Text(text) => {
                    let _ = state_clone
                        .broadcast
                        .send(Message::Custom(text.to_string()));
                }
                WsMessage::Close(_) => {
                    println!("Frontend disconnected!");
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

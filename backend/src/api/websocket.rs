use axum::{
    extract::{
        State,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};

use super::messages::{ClientMessage, ServerMessage};
use crate::{
    core::{
        database::craft_base_sql::get_craft_bases_by_class, database::item_sql::get_items_by_class,
        domain::template::Template,
    },
    optimization::worker::{OptimizationHandle, OptimizeStatus, start_optimization_worker},
    state::SharedState,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(socket: WebSocket, _state: SharedState) {
    let (mut sender, mut receiver) = socket.split();
    let (message_tx, mut message_rx) = mpsc::unbounded_channel::<ServerMessage>();

    let mut send_task = tokio::spawn(async move {
        while let Some(message) = message_rx.recv().await {
            let payload = match serde_json::to_string(&message) {
                Ok(payload) => payload,
                Err(error) => {
                    eprintln!("Failed to serialize websocket message: {error}");
                    continue;
                }
            };

            if sender.send(WsMessage::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    println!("Frontend connected!");

    let mut recv_task = tokio::spawn(async move {
        let mut active_optimization: Option<OptimizationHandle> = None;

        while let Some(Ok(message)) = receiver.next().await {
            if let WsMessage::Text(text) = message {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(ClientMessage::Start(req)) => match Template::try_from(req) {
                        Ok(template) => {
                            println!("Starting optimization for class {}", template.class.id());

                            if let Some(handle) = active_optimization.take() {
                                handle.cancel();
                            }

                            let connection = match _state.db_connection.lock() {
                                Ok(connection) => connection,
                                Err(error) => {
                                    let _ = message_tx.send(ServerMessage::Error {
                                        message: format!(
                                            "Failed to lock database connection: {error}"
                                        ),
                                    });
                                    continue;
                                }
                            };

                            let items = match get_items_by_class(&connection, template.class) {
                                Ok(items) => items,
                                Err(error) => {
                                    let _ = message_tx.send(ServerMessage::Error {
                                        message: error.to_string(),
                                    });
                                    continue;
                                }
                            };

                            let craft_bases =
                                match get_craft_bases_by_class(&connection, template.class) {
                                    Ok(craft_bases) => craft_bases,
                                    Err(error) => {
                                        let _ = message_tx.send(ServerMessage::Error {
                                            message: error.to_string(),
                                        });
                                        continue;
                                    }
                                };

                            let mut items = items;
                            items.extend(craft_bases.into_iter().map(Into::into));

                            let (status_tx, mut status_rx) =
                                mpsc::unbounded_channel::<OptimizeStatus>();
                            let message_tx_for_status = message_tx.clone();

                            tokio::spawn(async move {
                                while let Some(status) = status_rx.recv().await {
                                    let outbound = match status {
                                        OptimizeStatus::Setup => Some(ServerMessage::Setup),
                                        OptimizeStatus::Grounding => Some(ServerMessage::Grounding),
                                        OptimizeStatus::Solving => Some(ServerMessage::Solving),
                                        OptimizeStatus::NewModel(result) => {
                                            let equipped_items: HashMap<u16, u32> = result
                                                .template
                                                .slots
                                                .into_iter()
                                                .map(|(slot, item_id)| (slot.id(), item_id as u32))
                                                .collect();

                                            Some(ServerMessage::NewModel {
                                                equipped_items,
                                                equipped_gems: result
                                                    .equipped_gem_ids
                                                    .into_iter()
                                                    .map(|(item_id, gem_ids)| {
                                                        (item_id as u32, gem_ids)
                                                    })
                                                    .collect(),
                                            })
                                        }
                                        OptimizeStatus::Finished => Some(ServerMessage::Finished),
                                        OptimizeStatus::Error(message) => {
                                            Some(ServerMessage::Error { message })
                                        }
                                    };

                                    if let Some(outbound) = outbound {
                                        if message_tx_for_status.send(outbound).is_err() {
                                            break;
                                        }
                                    }
                                }
                            });

                            let handle = start_optimization_worker(template, items, status_tx);
                            active_optimization = Some(handle);
                        }
                        Err(e) => {
                            let _ = message_tx.send(ServerMessage::Error { message: e });
                        }
                    },
                    Ok(ClientMessage::Cancel) => {
                        println!("Canceling optimization...");

                        if let Some(handle) = active_optimization.take() {
                            handle.cancel();
                        }

                        let _ = message_tx.send(ServerMessage::Canceled);
                    }
                    Err(error) => {
                        println!("Error parsing websocket message: {error}");
                        let _ = message_tx.send(ServerMessage::Error {
                            message: error.to_string(),
                        });
                    }
                }
            }
        }

        if let Some(handle) = active_optimization.take() {
            handle.cancel();
        }

        println!("Frontend disconnected!");
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

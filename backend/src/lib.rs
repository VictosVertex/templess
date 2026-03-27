//! This crate aims to provide a comprehensive solution to the Dark Age of Camelot
//! Templating Problem.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

use std::{path::Path, sync::Arc, sync::Mutex};

use axum::{Router, routing::get};
use rusqlite::Connection;
use tokio::sync::broadcast;

use crate::{api::init, core::config::load_config, state::AppState};
pub use error::{Error, Result};
mod api;
pub mod clingo;
mod core;
mod error;
mod initialization;
mod message;
mod optimization;
mod state;

#[tokio::main]
pub async fn start() {
    let config = Arc::new(load_config("config.toml").expect("Failed to load configuration"));

    let db_path_str = config.database.path.clone();
    let db_exists = Path::new(&db_path_str).exists();

    let connection = Connection::open(&db_path_str).expect("Failed to open database");
    let db_connection = Arc::new(Mutex::new(connection));

    let (sender, _receiver) = broadcast::channel(1024);

    let app_state = AppState {
        config: config.clone(),
        db_connection: db_connection.clone(),
        template: Arc::new(Mutex::new(None)),
        items: Arc::new(Mutex::new(Vec::new())),
        broadcast: sender,
        is_initialized: db_exists,
    };

    let app = Router::new()
        .route("/ws", get(api::websocket::handler))
        .nest("/init", init::router())
        .with_state(Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();
}

//! This crate aims to provide a comprehensive solution to the Dark Age of Camelot
//! Templating Problem.

#![warn(missing_docs)]
pub mod api;
pub mod app_state;
pub mod core;
pub mod initialization;

use app_state::AppState;
use axum::Router;
use core::config::load_config;

/// Starts the application.
///
/// This function first creates the application state and then
/// builds the main router and starts the server.
///
/// # Returns
/// A result indicating whether the application started successfully or an error occurred.
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.toml").expect("Failed to load configuration");

    let connection =
        rusqlite::Connection::open(config.database.path.clone()).expect("Failed to open database");

    let state = AppState {
        config: std::sync::Arc::new(tokio::sync::Mutex::new(config)),
        db_connection: std::sync::Arc::new(tokio::sync::Mutex::new(connection)),
    };

    // Build the router with state
    let rt = Router::new()
        .merge(api::initialization::get_router().await)
        .with_state(state);

    // Serve the app
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, rt).into_future().await?;

    Ok(())
}

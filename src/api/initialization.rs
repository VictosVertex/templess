//! This module contains the initialization endpoints for handling database initialization.

use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::{app_state::AppState, core::database::schema::create_tables};
use std::time::Instant;

use crate::initialization::item_init::initialize_items;

/// Returns the router for the initialization endpoints.
///
/// All routes in this module should start with `/initialize`.
///
/// # Endpoints
/// - `/initialize`: Initializes the database by creating tables and populating initial data.
///
/// # Returns
/// A `Router` instance configured with the initialization routes.
pub async fn get_router() -> Router<AppState> {
    Router::new().route("/initialize", get(initialize_database))
}

/// Initializes the database by creating tables and populating initial data.
///
/// This function first creates the necessary database tables in the directory specified in the configuration file.
/// Then, it populates the database with initial items from the specified path in the configuration.
///
/// # Returns
/// An HTML response indicating the success of the database initialization.
pub async fn initialize_database(State(state): State<AppState>) -> impl IntoResponse {
    let config = state.config.lock().await;
    let mut connection = state.db_connection.lock().await;

    let start = Instant::now();
    print!("Creating tables ... ");
    create_tables(&connection).expect("Failed to create database tables");
    println!("done in {:.2?}", start.elapsed());

    let start = Instant::now();
    println!("Initializing items ... ");
    initialize_items(&mut connection, config.data.items_path.clone())
        .expect("Failed to initialize items");
    println!("done in {:.2?}", start.elapsed());

    Html("<h1>Database initialized successfully!</h1>").into_response()
}

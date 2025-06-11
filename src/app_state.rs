//! This module defines the AppState struct, which holds the application state.
//!
//! It is used to share state such as configuration and database connections
//! across different requests.

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::core::{
    config::Config,
    domain::{item::Item, template::Template},
};

/// This struct holds the application state, including configuration,
#[derive(Clone)]
pub struct AppState {
    /// The application configuration.
    pub config: Arc<Config>,

    /// The database connection.
    pub db_connection: Arc<Mutex<Connection>>,

    /// The currently worked on template.
    pub template: Arc<Mutex<Option<Template>>>,

    /// All items that can be used in the current template.
    pub items: Arc<Mutex<Vec<Arc<Item>>>>,
}

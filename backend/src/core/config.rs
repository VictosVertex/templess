//! This module defines the configuration structure and loading functionality for the application.
//!
//! It uses the `serde` crate for deserialization and `toml` for parsing configuration files.

use serde::Deserialize;
use std::error::Error;

/// This struct represents the configuration for the application.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The database configuration section.
    #[serde(rename = "database")]
    pub database: DatabaseConfig,

    /// The data configuration section.
    #[serde(rename = "data")]
    pub data: DataConfig,
}

/// This struct represents the database configuration section of the application.
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    /// The path to the SQLite database file.
    pub path: String,
}

/// This struct represents the data configuration section of the application.
#[derive(Debug, Deserialize)]
pub struct DataConfig {
    /// The path to the items (json) data file.
    pub items_path: String,
}

/// Loads the configuration from a TOML file at the specified path.
///
/// # Parameters
/// - `path`: The path to the configuration file.
///
/// # Returns
/// A `Result` containing the parsed `Config` or an error if the file could not be read or parsed.
pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_str = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

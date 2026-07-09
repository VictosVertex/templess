//! This module contains the initialization logic for preference presets.
//!
//! It reads raw preset data from a JSON file, converts it to the application's `PreferencePreset` type, 
//! and inserts it into the database.

use std::{fs::File, io::BufReader};

use super::raw_preference_preset::RawPreferencePreset;
use crate::core::error::CoreResult;
use crate::core::{
    database::preference_preset_sql::insert_preference_presets, // Adjust import to your actual SQL file
    domain::preference_preset::PreferencePreset,
};

/// Initializes preference presets in the database from a JSON file.
///
/// # Parameters
/// - `connection`: A mutable reference to the SQLite connection.
/// - `data_path`: The path to the JSON file containing raw preset data.
///
/// # Returns
/// A `Result` indicating success or failure. If successful, it returns `Ok(())`.
/// If an error occurs, it returns an `anyhow::Error` containing the error details.
pub fn initialize_preference_presets(
    connection: &mut rusqlite::Connection,
    data_path: String,
) -> CoreResult<()> {
    let file = File::open(&data_path)?;
    let reader = BufReader::new(file);
    let raw_presets: Vec<RawPreferencePreset> = serde_json::from_reader(reader)?;

    println!("Found {} raw preference presets", raw_presets.len());

    // Using enumerate to pass a sequential ID (starting at 1) to the domain constructor
    let presets = raw_presets
        .into_iter()
        .enumerate()
        .filter_map(|(index, raw): (usize, RawPreferencePreset)| raw.into_preset((index + 1) as u32))
        .collect::<Vec<PreferencePreset>>();

    println!("Converted to {} preference presets", presets.len());

    insert_preference_presets(connection, &presets)?;

    Ok(())
}
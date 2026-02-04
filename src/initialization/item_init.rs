//! This module contains the initialization logic for items.
//!
//! It reads raw item data from a JSON file, converts it to the application's `Item` type, and inserts it into the database.

use std::{fs::File, io::BufReader};

use super::raw_item::RawItem;
use crate::core::{database::item_sql::insert_items, domain::item::Item};
use anyhow::Result;

/// Initializes items in the database from a JSON file.
///
/// # Parameters
/// - `connection`: A mutable reference to the SQLite connection.
/// - `data_path`: The path to the JSON file containing raw item data.
///
/// # Returns
/// A `Result` indicating success or failure. If successful, it returns `Ok(())`.
/// If an error occurs, it returns an `anyhow::Error` containing the error details.
pub fn initialize_items(connection: &mut rusqlite::Connection, data_path: String) -> Result<()> {
    let file = File::open(&data_path)?;
    let reader = BufReader::new(file);
    let raw_items: Vec<RawItem> = serde_json::from_reader(reader)?;

    println!("Found {} raw items", raw_items.len());
    let items = raw_items
        .into_iter()
        .filter_map(RawItem::into_item)
        .collect::<Vec<Item>>();

    println!("Converted to {} items", items.len());

    insert_items(connection, items)?;

    Ok(())
}

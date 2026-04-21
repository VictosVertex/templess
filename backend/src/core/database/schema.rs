//! This module contains the database schema creation functionality.
//!
//! It defines the structure of the database tables used in the application.

use crate::core::error::CoreResult;

use rusqlite::Connection;

/// Creates the necessary database tables if they do not already exist.
///
/// # Parameters
/// - `connection`: A reference to the database connection.
///
/// # Returns
/// - `Ok(())` if the tables were created successfully.
/// - `Err(Box<dyn Error>)` if an error occurred during the table creation.
pub fn create_tables(connection: &Connection) -> CoreResult<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS item (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            model INTEGER NOT NULL,
            object_type INTEGER NOT NULL,
            item_type INTEGER NOT NULL,
            level INTEGER NOT NULL,
            quality INTEGER NOT NULL,
            weapon_hand INTEGER NOT NULL,
            weapon_speed INTEGER NOT NULL,
            damage_type INTEGER NOT NULL,
            realm INTEGER NOT NULL,
            required_level INTEGER NOT NULL,
            bonus_level INTEGER NOT NULL,
            shield_size INTEGER NOT NULL,
            instrument_type INTEGER NOT NULL,
            is_tradable INTEGER NOT NULL,
            utility_single REAL NOT NULL,
            utility REAL NOT NULL
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS item_class (
            item_id INTEGER NOT NULL,
            class_id INTEGER NOT NULL,
            PRIMARY KEY (item_id, class_id),
            FOREIGN KEY(item_id) REFERENCES item(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS item_stat (
            item_id INTEGER NOT NULL,
            stat_id INTEGER NOT NULL,
            value INTEGER NOT NULL,
            PRIMARY KEY (item_id, stat_id),
            FOREIGN KEY(item_id) REFERENCES item(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS template (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            class_id INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS template_slot (
            template_id INTEGER NOT NULL,
            slot_id INTEGER NOT NULL,
            item_id INTEGER NOT NULL,
            PRIMARY KEY (template_id, slot_id),
            FOREIGN KEY(template_id) REFERENCES template(id) ON DELETE CASCADE,
            FOREIGN KEY(item_id) REFERENCES item(id)
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS template_preference (
            template_id INTEGER NOT NULL,
            stat_id INTEGER NOT NULL,
            lower_bound INTEGER NOT NULL,
            upper_bound INTEGER NOT NULL,
            priority INTEGER NOT NULL,
            PRIMARY KEY (template_id, stat_id),
            FOREIGN KEY(template_id) REFERENCES template(id) ON DELETE CASCADE
        )",
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS craft_base (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            item_type INTEGER NOT NULL,
            item_slot INTEGER NOT NULL,
            realm INTEGER NOT NULL,
            stat_id INTEGER NOT NULL,
            value INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(())
}

/// Checks if the database has been initialized by verifying the existence of the `item` table.
///
/// # Parameters
/// - `connection`: A reference to the database connection.
pub fn is_initialized(connection: &Connection) -> CoreResult<bool> {
    let mut stmt =
        connection.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='item'")?;
    let mut rows = stmt.query([])?;

    Ok(rows.next()?.is_some())
}

//! This module contains the sql queries for handling items in the database.

use crate::core::domain::{class::Class, item::Item, item_type::ItemType, realm::Realm};
use rusqlite::{Connection, params};
use std::error::Error;

/// Inserts a vector of items into the database.
///
/// This function starts a transaction, prepares the necessary SQL statements,
/// and executes the insertions for each item, including its allowed classes and bonuses.
///
/// # Arguments
/// - `connection`: A mutable reference to the database connection.
/// - `items`: A vector of `Item` objects to be inserted into the database.
///
/// # Returns
/// - `Ok(())` if the insertion was successful.
/// - `Err(Box<dyn Error>)` if an error occurred during the insertion.
pub fn insert_items(connection: &mut Connection, items: Vec<Item>) -> Result<(), Box<dyn Error>> {
    let transaction = connection.transaction()?;

    {
        let mut item_insert = transaction.prepare(
            "INSERT INTO item (
                id,             name,           model,              object_type,
                item_type,      level,          quality,            weapon_hand,
                weapon_speed,   damage_type,    realm,              required_level,
                bonus_level,    shield_size,    instrument_type,    is_tradable,
                utility_single, utility
            ) VALUES (
                ?,  ?,  ?,  ?,  ?,  ?,  ?,  ?, ?,
                ?,  ?,  ?,  ?,  ?,  ?,  ?,  ?, ?
            )",
        )?;

        let mut class_insert =
            transaction.prepare("INSERT INTO item_class (item_id, class_id) VALUES (?, ?)")?;
        let mut bonus_insert = transaction
            .prepare("INSERT INTO item_stat (item_id, stat_id, value) VALUES (?, ?, ?)")?;

        for item in items {
            item_insert.execute(params![
                item.id,
                item.name,
                item.model,
                item.object_type,
                item.item_type.id(),
                item.level,
                item.quality,
                item.weapon_hand,
                item.weapon_speed,
                item.damage_type,
                item.realm.id(),
                item.required_level,
                item.bonus_level,
                item.shield_size,
                item.instrument_type,
                if item.is_tradable { 1 } else { 0 },
                item.utility_single,
                item.utility,
            ])?;

            // Insert allowed classes
            for class in &item.allowed_classes {
                class_insert.execute(params![item.id, class.id()])?;
            }
            // Insert bonuses
            for bonus in &item.bonuses {
                bonus_insert.execute(params![item.id, bonus.stat.id(), bonus.value])?;
            }
        }
    }

    transaction.commit()?;

    Ok(())
}

/// Retrieves items from the database based on the specified class.
///
/// This function queries the `item` table and joins it with the `item_class` table to filter items
/// that belong to the specified class or have no associated class. Because items that are not associated
/// with any class may be from a different realm, it also checks the realm of the class.
///
/// # Parameters
/// - `connection`: A reference to the database connection.
/// - `class`: The `Class` for which to retrieve items.
///
/// # Returns
/// - `Ok(Vec<Item>)`: A vector of `Item` objects that match the specified class.
/// - `Err(Box<dyn Error>)`: If an error occurs during the query execution.
pub fn get_items_by_class(
    connection: &Connection,
    class: Class,
) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut stmt = connection.prepare(
        "SELECT 
            i.id, 
            i.name, 
            i.model, 
            i.object_type, 
            i.item_type, 
            i.level, 
            i.quality,
            i.weapon_hand, 
            i.weapon_speed, 
            i.damage_type, 
            i.realm, 
            i.required_level,
            i.bonus_level, 
            i.shield_size, 
            i.instrument_type, 
            i.is_tradable,
            i.utility_single, 
            i.utility
         FROM 
            item i
         LEFT JOIN 
            item_class ic ON ic.item_id = i.id
         WHERE 
            ic.class_id = ?
            OR NOT EXISTS (
            SELECT 1 FROM item_class ic2 WHERE ic2.item_id = i.id
            )
            AND (i.realm = ? OR i.realm = 0)",
    )?;
    let class_id = class.id();
    let realm_id = class.realm().id();
    println!(
        "Querying items for class {:?} (id: {}, realm: {})",
        class, class_id, realm_id
    );
    let items = stmt.query_map(params![class_id, realm_id], |row| {
        let item_type = ItemType::from_repr(row.get::<_, u16>(4)?).expect("Invalid item_type repr");
        let realm = Realm::from_repr(row.get::<_, u16>(10)?).expect("Invalid realm repr");

        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            model: row.get(2)?,
            object_type: row.get(3)?,
            item_type,
            level: row.get(5)?,
            quality: row.get(6)?,
            weapon_hand: row.get(7)?,
            weapon_speed: row.get(8)?,
            damage_type: row.get(9)?,
            realm,
            required_level: row.get(11)?,
            bonus_level: row.get(12)?,
            shield_size: row.get(13)?,
            instrument_type: row.get(14)?,
            is_tradable: row.get::<_, u8>(15)? != 0,
            utility_single: row.get(16)?,
            utility: row.get(17)?,
            allowed_classes: vec![], // This should be filled later if needed
            bonuses: vec![],         // This should be filled later if needed
            proc1_json: None,
            proc2_json: None,
            use1_json: None,
            use2_json: None,
            passive_json: None,
            react1_json: None,
            react2_json: None, // This should be filled later if needed
        })
    })?;

    Ok(items.collect::<Result<Vec<_>, _>>()?)
}

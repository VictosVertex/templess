//! Initialization logic for craft bases.

use std::{fs::File, io::BufReader};

use super::raw_craft_base::RawCraftBase;
use crate::core::error::CoreResult;
use crate::core::{database::craft_base_sql::insert_craft_bases, domain::craft_base::CraftBase};

const CRAFT_BASE_ID_START: i32 = 900_000;

/// Initializes craft bases in the database from a JSON file.
pub fn initialize_craft_bases(
    connection: &mut rusqlite::Connection,
    data_path: String,
) -> CoreResult<()> {
    let file = File::open(&data_path)?;
    let reader = BufReader::new(file);
    let raw_items: Vec<RawCraftBase> = serde_json::from_reader(reader)?;

    println!("Found {} raw craft bases", raw_items.len());
    let craft_bases = raw_items
        .into_iter()
        .enumerate()
        .filter_map(|(index, raw_item)| {
            raw_item.into_craft_base(CRAFT_BASE_ID_START + index as i32)
        })
        .collect::<Vec<CraftBase>>();

    println!("Converted to {} craft bases", craft_bases.len());

    insert_craft_bases(connection, craft_bases)?;

    Ok(())
}

//! This module contains the SQL queries for handling craft bases in the database.

use crate::core::{
    domain::{
        class::Class, craft_base::CraftBase, item_bonus::ItemBonus, item_slot::ItemSlot,
        item_type::ItemType, realm::Realm, stat::Stat,
    },
    error::CoreResult,
};

use rusqlite::{Connection, params};

/// Inserts a vector of craft bases into the database.
pub fn insert_craft_bases(
    connection: &mut Connection,
    craft_bases: Vec<CraftBase>,
) -> CoreResult<()> {
    let transaction = connection.transaction()?;

    {
        let mut craft_base_insert = transaction.prepare(
            "INSERT INTO craft_base (
				id, name, item_type, item_slot, realm, stat_id, value
			) VALUES (
				?, ?, ?, ?, ?, ?, ?
			)",
        )?;

        for craft_base in craft_bases {
            craft_base_insert.execute(params![
                craft_base.id,
                craft_base.name,
                craft_base.item_type.id(),
                craft_base.item_slot.id(),
                craft_base.realm.id(),
                craft_base.base_bonus.stat.id(),
                craft_base.base_bonus.value,
            ])?;
        }
    }

    transaction.commit()?;

    Ok(())
}

pub fn get_craft_bases_by_class(
    connection: &Connection,
    class: Class,
) -> CoreResult<Vec<CraftBase>> {
    let mut stmt = connection.prepare(
        "SELECT
			cb.id,
			cb.name,
			cb.item_type,
			cb.item_slot,
			cb.realm,
			cb.stat_id,
			cb.value
		 FROM craft_base cb
		 WHERE cb.realm = ? OR cb.realm = 0",
    )?;

    let realm_id = class.realm().id();
    let craft_bases = stmt.query_map(params![realm_id], |row| {
        let item_type = ItemType::from_repr(row.get::<_, u16>(2)?)
            .expect("Invalid item_type repr from craft_base");
        let item_slot = ItemSlot::from_repr(row.get::<_, u16>(3)?)
            .expect("Invalid item_slot repr from craft_base");
        let realm =
            Realm::from_repr(row.get::<_, u16>(4)?).expect("Invalid realm repr from craft_base");
        let stat_id = row.get::<_, u16>(5)?;
        let value = row.get::<_, u16>(6)?;

        Ok(CraftBase {
            id: row.get(0)?,
            name: row.get(1)?,
            item_type,
            item_slot,
            realm,
            base_bonus: ItemBonus {
                stat: Stat::from_repr(stat_id).expect("Invalid stat ID from craft_base"),
                value,
            },
        })
    })?;

    let potential_bases = craft_bases.collect::<Result<Vec<_>, _>>()?;
    let allowed_types = class.allowed_item_types();

    Ok(potential_bases
        .into_iter()
        .filter(|craft_base| allowed_types.contains(&craft_base.item_type))
        .collect())
}

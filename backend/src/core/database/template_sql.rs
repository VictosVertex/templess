use std::collections::HashMap;

use rusqlite::{Connection, params};

use crate::core::{
    domain::{
        class::Class,
        equip::{EquipSource, EquippedItemState},
        item_slot::ItemSlot,
        preference::Preference,
        template::Template,
    },
    error::CoreResult,
};

pub fn insert_template(
    connection: &Connection,
    name: String,
    class_id: u16,
    preference_preset_id: Option<u32>,
) -> CoreResult<i32> {
    connection.execute(
        "INSERT INTO template (name, class_id) VALUES (?, ?)",
        params![name, class_id],
    )?;

    let template_id = connection.last_insert_rowid() as i32;

    if let Some(preset_id) = preference_preset_id {
        connection.execute(
            "INSERT INTO template_preference (template_id, stat_id, min, weight)
             SELECT ?, p.stat_id, p.min, p.weight
             FROM preference p
             INNER JOIN preference_preset pp ON pp.id = p.preset_id
             WHERE p.preset_id = ? AND pp.class_id = ?",
            params![template_id, preset_id, class_id],
        )?;
    }

    Ok(template_id)
}

fn get_template_slot_gems(
    connection: &Connection,
    template_id: i32,
) -> CoreResult<HashMap<u16, Vec<u32>>> {
    let mut stmt = connection.prepare(
        "SELECT slot_id, gem_id FROM template_slot_gem WHERE template_id = ? ORDER BY slot_id, gem_id",
    )?;
    let rows = stmt.query_map(params![template_id], |row| {
        Ok((row.get::<_, u16>(0)?, row.get::<_, u32>(1)?))
    })?;

    let mut gems_by_slot: HashMap<u16, Vec<u32>> = HashMap::new();

    for row in rows {
        let (slot_id, gem_id) = row?;
        gems_by_slot.entry(slot_id).or_default().push(gem_id);
    }

    Ok(gems_by_slot)
}

fn get_template_preferences(
    connection: &Connection,
    template_id: i32,
) -> CoreResult<HashMap<u16, Preference>> {
    let mut stmt = connection.prepare(
        "SELECT stat_id, min, weight FROM template_preference WHERE template_id = ?",
    )?;
    let rows = stmt.query_map(params![template_id], |row| {
        Ok((
            row.get::<_, u16>(0)?,
            Preference {
                min: row.get::<_, u16>(1)?,
                weight: row.get::<_, u16>(2)?,
            },
        ))
    })?;

    rows.collect::<Result<HashMap<_, _>, _>>().map_err(Into::into)
}

pub fn get_templates(connection: &Connection) -> CoreResult<Vec<Template>> {
    let mut stmt = connection.prepare("SELECT id, name, class_id FROM template")?;
    let template_rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, u16>(2)?,
        ))
    })?;

    let mut templates = Vec::new();

    for template_row in template_rows {
        let (id, name, class_id) = template_row?;
        let preferences = get_template_preferences(connection, id)?;

        if let Some(class) = Class::from_repr(class_id) {
            templates.push(Template {
                id,
                name,
                class,
                equipped_items: HashMap::new(),
                preferences,
            });
        }
    }

    Ok(templates)
}

pub fn get_template(connection: &Connection, template_id: i32) -> CoreResult<Option<Template>> {
    let mut stmt = connection.prepare("SELECT id, name, class_id FROM template WHERE id = ?")?;
    let template_row = stmt.query_row(params![template_id], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, u16>(2)?,
        ))
    });

    if let Ok((id, name, class_id)) = template_row {
        let preferences = get_template_preferences(connection, id)?;
        let mut gems_by_slot = get_template_slot_gems(connection, id)?;
        let mut slot_stmt = connection
            .prepare("SELECT slot_id, item_id, source FROM template_slot WHERE template_id = ?")?;
        let slot_rows = slot_stmt.query_map(params![id], |row| {
            Ok((
                row.get::<_, u16>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, u8>(2)?,
            ))
        })?;

        let mut equipped_items = HashMap::new();
        for slot_row in slot_rows {
            let (slot_id, item_id, source) = slot_row?;
            if let Some(slot) = ItemSlot::from_repr(slot_id) {
                equipped_items.insert(
                    slot,
                    EquippedItemState {
                        item_id: item_id as u32,
                        source: match source {
                            0 => EquipSource::User,
                            1 => EquipSource::Optimizer,
                            _ => EquipSource::User,
                        },
                        gem_ids: gems_by_slot.remove(&slot_id).unwrap_or_default(),
                    },
                );
            }
        }

        if let Some(class) = Class::from_repr(class_id) {
            return Ok(Some(Template {
                id,
                name,
                class,
                equipped_items,
                preferences,
            }));
        }
    }

    Ok(None)
}

pub fn delete_template(connection: &Connection, template_id: i32) -> CoreResult<()> {
    connection.execute("DELETE FROM template WHERE id = ?", params![template_id])?;
    Ok(())
}

pub fn update_template(
    connection: &Connection,
    template_id: i32,
    preferences: HashMap<u16, Preference>,
    equipped_items: HashMap<ItemSlot, EquippedItemState>,
) -> CoreResult<Template> {
    let tx = connection.unchecked_transaction()?;

    tx.execute(
        "DELETE FROM template_slot_gem WHERE template_id = ?",
        params![template_id],
    )?;
    tx.execute(
        "DELETE FROM template_slot WHERE template_id = ?",
        params![template_id],
    )?;
    tx.execute(
        "DELETE FROM template_preference WHERE template_id = ?",
        params![template_id],
    )?;

    {
        let mut pref_stmt = tx.prepare(
            "INSERT INTO template_preference (template_id, stat_id, min, weight) VALUES (?, ?, ?, ?)",
        )?;

        for (stat_id, preference) in &preferences {
            pref_stmt.execute(params![template_id, stat_id, preference.min, preference.weight])?;
        }
    }

    {
        let mut slot_stmt = tx.prepare(
            "INSERT INTO template_slot (template_id, slot_id, item_id, source) VALUES (?, ?, ?, ?)",
        )?;
        let mut gem_stmt = tx.prepare(
            "INSERT INTO template_slot_gem (template_id, slot_id, gem_id) VALUES (?, ?, ?)",
        )?;

        for (slot, equipped_item) in &equipped_items {
            let source = match equipped_item.source {
                EquipSource::User => 0,
                EquipSource::Optimizer => 1,
            };

            slot_stmt.execute(params![template_id, slot.id(), equipped_item.item_id, source])?;

            for gem_id in &equipped_item.gem_ids {
                gem_stmt.execute(params![template_id, slot.id(), gem_id])?;
            }
        }
    }

    tx.commit()?;

    get_template(connection, template_id)?.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows.into())
}

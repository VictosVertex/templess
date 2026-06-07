use std::collections::HashMap;

use rusqlite::{Connection, params};

use crate::core::{
    domain::{
        class::Class,
        equip::{EquipSource, EquippedItemState},
        item_slot::ItemSlot,
        template::Template,
    },
    error::CoreResult,
};

pub fn insert_template(connection: &Connection, name: String, class_id: u16) -> CoreResult<i32> {
    connection.execute(
        "INSERT INTO template (name, class_id) VALUES (?, ?)",
        params![name, class_id],
    )?;

    Ok(connection.last_insert_rowid() as i32)
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

        if let Some(class) = Class::from_repr(class_id) {
            templates.push(Template {
                id,
                name,
                class,
                equipped_items: HashMap::new(),
                preferences: HashMap::new(),
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
                        gem_ids: Vec::new(), // TODO: actually load gems
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
                preferences: HashMap::new(),
            }));
        }
    }

    Ok(None)
}

pub fn delete_template(connection: &Connection, template_id: i32) -> CoreResult<()> {
    connection.execute("DELETE FROM template WHERE id = ?", params![template_id])?;
    Ok(())
}

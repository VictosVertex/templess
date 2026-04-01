use std::collections::HashMap;

use rusqlite::{Connection, params};

use crate::core::{
    domain::{class::Class, item_slot::ItemSlot, template::Template},
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

        let mut slot_stmt = connection
            .prepare("SELECT slot_id, item_id FROM template_slot WHERE template_id = ?")?;
        let slot_rows = slot_stmt.query_map(params![id], |row| {
            Ok((row.get::<_, u16>(0)?, row.get::<_, i32>(1)?))
        })?;

        let mut slots = HashMap::new();
        for slot_row in slot_rows {
            let (slot_id, item_id) = slot_row?;
            if let Some(slot) = ItemSlot::from_repr(slot_id) {
                slots.insert(slot, item_id);
            }
        }

        if let Some(class) = Class::from_repr(class_id) {
            templates.push(Template {
                id,
                name,
                class,
                slots,
            });
        }
    }

    Ok(templates)
}

pub fn delete_template(connection: &Connection, template_id: i32) -> CoreResult<()> {
    connection.execute("DELETE FROM template WHERE id = ?", params![template_id])?;
    Ok(())
}

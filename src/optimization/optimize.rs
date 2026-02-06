use crate::{core::domain::{item::Item, item_slot::ItemSlot, template::Template}, optimization::instance::{class_atoms, item_atoms, stat_atoms, slot_atoms}};
use std::{path::Path};
use anyhow::Result;
use std::sync::Arc;

pub fn optimize(template: Template, items: Vec<Arc<Item>>) -> Result<Template> {
    println!("Optimization started for class: {:?}", template.class);

    let relevant_items: Vec<Arc<Item>> = items
        .iter()
        .filter(|item| match &item.item_slot {
            ItemSlot::Bracer => {
                !(template.slots.contains_key(&ItemSlot::Bracer)
                    && template.slots.contains_key(&ItemSlot::Bracer2))
            }
            ItemSlot::Ring => {
                !(template.slots.contains_key(&ItemSlot::Ring)
                    && template.slots.contains_key(&ItemSlot::Ring2))
            }
            _ => !template.slots.contains_key(&item.item_slot),
        })
        .cloned()
        .collect();

    if relevant_items.is_empty() {
        println!("No relevant items found (Template might be full).");
        return Ok(template)
    }
    println!("Found {} relevant items to optimize.", relevant_items.len());

    let mut asp_data = String::new();
    asp_data.push_str(&class_atoms(template.class)?);
    asp_data.push_str(&slot_atoms(&template)?);
    asp_data.push_str(&stat_atoms()?);
    asp_data.push_str(&item_atoms(&items)?);

    let file_path = Path::new("instance.lp");
    std::fs::write(file_path, &asp_data)?;
    
    println!("ASP instance written to {:?}", file_path.canonicalize().unwrap_or(file_path.to_path_buf()));

    Ok(template)
}
use crate::core::domain::{
    class::Class, item::Item, item_slot::ItemSlot, stat::Stat, template::Template,
};
use anyhow::Result;
use std::fmt::Write;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub fn item_atoms(items: &[Arc<Item>]) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- AVAILABLE ITEMS ---")?;

    for item in items {
        writeln!(
            asp,
            "item({}, {}, \"{}\").",
            item.id,
            item.item_slot.name(),
            item.name
        )?;

        for bonus in &item.bonuses {
            let stat_name = bonus.stat.to_string().to_lowercase();
            writeln!(
                asp,
                "item_bonus({}, {}, {}).",
                item.id, stat_name, bonus.value
            )?;
        }
    }
    Ok(asp)
}

pub fn stat_atoms() -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- STATS ---")?;
    for stat in Stat::iter() {
        writeln!(
            asp,
            "stat({}, {}, {}).",
            stat.name(),
            (stat.utility_per_point() * 100.0).round() as i32,
            stat.cap()
        )?;

        if let Some(cap) = stat.cap_stat() {
            writeln!(asp, "stat_cap({}, {}).", stat.name(), cap.name())?;
        }

        if let Some(base) = stat.base_stat() {
            writeln!(asp, "stat_base({}, {}).", stat.name(), base.name())?;
        }
    }
    Ok(asp)
}

pub fn class_atoms(class: Class) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- CLASS ---")?;

    writeln!(asp, "class({}).", class.to_string().to_lowercase())?;

    for line in class.skill_lines() {
        writeln!(asp, "class_skill_line({}).", line)?;
    }

    if let Some(acuity) = class.acuity_stat() {
        writeln!(asp, "class_acuity({}).", acuity)?;
    }

    Ok(asp)
}

pub fn slot_atoms(template: &Template) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- TEMPLATE ---")?;

    for slot in ItemSlot::iter() {
        writeln!(asp, "slot({},{}).", slot.id(), slot.name())?;

        if let Some(item) = template.slots.get(&slot) {
            writeln!(asp, "slot_taken({}, {}).", slot.id(), item.id)?;
        }
    }

    Ok(asp)
}

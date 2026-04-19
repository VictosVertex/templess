//! This module provides functions for generating ASP atoms for the optimization.

use crate::core::domain::preference::Preference;
use crate::core::domain::{
    class::Class, item::Item, item_slot::ItemSlot, stat::Stat, template::Template,
};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::sync::Arc;
use strum::IntoEnumIterator;

/// Generates item related ASP atoms.
///
/// # Errors
/// - `Err(anyhow::Error)` if an error occurs during atom generation.
pub fn item_atoms(items: &[Arc<Item>], template: &Template) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- AVAILABLE ITEMS ---")?;

    for item in items {
        if template.slots.contains_key(&item.item_slot) {
            continue;
        }

        let unneeded_bonuses = item
            .bonuses
            .iter()
            .filter(|bonus| {
                let mut is_preferred = template.preferences.contains_key(&bonus.stat.id());

                if bonus.stat == Stat::Acuity || bonus.stat == Stat::AcuityCap {
                    is_preferred = match template.class.acuity_stat() {
                        Some(stat) => template.preferences.contains_key(&stat.id()),
                        None => false,
                    };
                }

                !is_preferred
            })
            .count();

        if unneeded_bonuses > 2 {
            continue;
        }

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

/// Generates stat related ASP atoms.
///
/// # Errors
/// - `Err(anyhow::Error)` if an error occurs during atom generation.
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

/// Generates class related ASP atoms.
///
/// # Errors
/// - `Err(anyhow::Error)` if an error occurs during atom generation.
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

/// Generates item slot related ASP atoms.
///
/// # Errors
/// - `Err(anyhow::Error)` if an error occurs during atom generation.
pub fn slot_atoms(template: &Template) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- TEMPLATE ---")?;

    for slot in ItemSlot::iter() {
        match template.slots.get(&slot) {
            Some(_) => {}
            None => writeln!(asp, "slot({},{}).", slot.id(), slot.name())?,
        }
    }

    Ok(asp)
}

pub fn preference_atoms(preferences: &HashMap<u16, Preference>) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- PREFERENCES ---")?;

    for (stat_id, preference) in preferences {
        if preference.weight > 0 {
            let Some(stat) = Stat::from_repr(*stat_id) else {
                return Err(anyhow::anyhow!("Invalid stat ID: {}", stat_id));
            };

            writeln!(
                asp,
                "preference({}, {}, {}).",
                stat.name(),
                preference.min,
                preference.weight
            )?
        }
    }

    Ok(asp)
}

pub fn stat_baseline_atoms(template: &Template, items: &[Arc<Item>]) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- STAT BASELINE ---")?;

    let mut baselines: HashMap<Stat, u16> = template
        .preferences
        .iter()
        .filter(|(_, pref)| pref.weight > 0)
        .filter_map(|(stat_id, _)| Stat::from_repr(*stat_id))
        .map(|stat| (stat, 0))
        .collect();

    let equipped_item_ids: HashSet<i32> = template.slots.values().copied().collect();

    let class_acuity = template.class.acuity_stat();
    let class_acuity_cap = class_acuity.and_then(|stat| stat.cap_stat());

    for item in items.iter().filter(|i| equipped_item_ids.contains(&i.id)) {
        for bonus in &item.bonuses {
            let actual_stat = match bonus.stat {
                Stat::Acuity => class_acuity,
                Stat::AcuityCap => class_acuity_cap,
                stat => Some(stat),
            };

            let Some(stat) = actual_stat else {
                continue;
            };

            if let Some(baseline) = baselines.get_mut(&stat) {
                *baseline += bonus.value;
            }
        }
    }

    for (stat, baseline) in baselines {
        writeln!(asp, "stat_baseline({}, {}).", stat.name(), baseline)?;
    }

    Ok(asp)
}

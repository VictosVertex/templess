//! This module provides functions for generating ASP atoms for the optimization.

use crate::core::domain::gem::Gem;
use crate::core::domain::item::ItemSource;
use crate::core::domain::preference::Preference;
use crate::core::domain::stat_category::StatCategory;
use crate::core::domain::{
    class::Class, item::Item, item_slot::ItemSlot, stat::Stat, template::Template,
};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use strum::IntoEnumIterator;

fn item_slot_is_available(template: &Template, item_id: i32, item_slot: ItemSlot) -> bool {
    if template
        .slots
        .values()
        .any(|&fixed_item_id| fixed_item_id == item_id)
    {
        return false;
    }

    match item_slot {
        ItemSlot::Ring | ItemSlot::Ring2 => {
            !template.slots.contains_key(&ItemSlot::Ring)
                || !template.slots.contains_key(&ItemSlot::Ring2)
        }
        ItemSlot::Bracer | ItemSlot::Bracer2 => {
            !template.slots.contains_key(&ItemSlot::Bracer)
                || !template.slots.contains_key(&ItemSlot::Bracer2)
        }
        _ => !template.slots.contains_key(&item_slot),
    }
}

fn resolved_bonus_stats(stat: Stat, class: Class) -> Vec<Stat> {
    match stat {
        Stat::Acuity => class.acuity_stat().into_iter().collect(),
        Stat::AcuityCap => class
            .acuity_stat()
            .and_then(|stat| stat.cap_stat())
            .into_iter()
            .collect(),
        Stat::AllMagicSkills => class
            .skill_lines()
            .iter()
            .filter(|line| matches!(line.category(), StatCategory::MagicSkills))
            .cloned()
            .collect(),

        Stat::AllMeleeSkills => class
            .skill_lines()
            .iter()
            .filter(|line| matches!(line.category(), StatCategory::MeleeSkills))
            .cloned()
            .collect(),

        Stat::AllArcherySkills => class
            .skill_lines()
            .iter()
            .filter(|line| matches!(line.category(), StatCategory::ArcherySkills))
            .cloned()
            .collect(),
        Stat::AllDualWieldingSkills => class
            .skill_lines()
            .iter()
            .filter(|line| matches!(line.category(), StatCategory::DualWieldingSkills))
            .cloned()
            .collect(),

        _ => vec![stat],
    }
}

/// Generates item related ASP atoms.
///
/// # Errors
/// - `Err(anyhow::Error)` if an error occurs during atom generation.
pub fn item_atoms(items: &[Item], template: &Template) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- AVAILABLE ITEMS ---")?;

    let mut item_buffer = String::with_capacity(1024);

    for item in items {
        if !item_slot_is_available(template, item.id, item.item_slot) {
            continue;
        }

        item_buffer.clear();

        writeln!(
            item_buffer,
            "item({}, {}, \"{}\").",
            item.id,
            item.item_slot.name(),
            item.name
        )?;

        if item.source == ItemSource::Crafted {
            writeln!(item_buffer, "crafted_item({}).", item.id)?;
        }

        let mut has_preferred_bonus = false;
        let mut utility = 0u32;

        for bonus in &item.bonuses {
            let expanded_stats = resolved_bonus_stats(bonus.stat, template.class);

            let is_preferred = expanded_stats
                .iter()
                .any(|stat| template.preferences.contains_key(&stat.id()));

            if is_preferred {
                has_preferred_bonus = true;

                // We still use the normal stat from the bonus here for now,
                // as currently our encoding does the mapping anyways.
                let stat_name = bonus.stat.to_string().to_lowercase();
                writeln!(
                    item_buffer,
                    "item_bonus({}, {}, {}).",
                    item.id, stat_name, bonus.value
                )?;

                let utility_per_point: f32 = expanded_stats
                    .iter()
                    .map(|stat| stat.utility_per_point())
                    .sum();

                utility += (utility_per_point * 100.0).round() as u32 * u32::from(bonus.value);
            }
        }

        if has_preferred_bonus {
            writeln!(item_buffer, "item_utility({}, {}).", item.id, utility)?;
            asp.push_str(&item_buffer);
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
        let gem_category = match stat.category() {
            StatCategory::PhysicalStats | StatCategory::AcuityStats => {
                if stat == Stat::Hitpoints {
                    Some("hitpoints")
                } else {
                    Some("stat")
                }
            }
            StatCategory::Resists => Some("resists"),
            StatCategory::MagicSkills
            | StatCategory::MeleeSkills
            | StatCategory::OtherSkills
            | StatCategory::DualWieldingSkills
            | StatCategory::ArcherySkills => Some("skills"),
            _ => None,
        };

        writeln!(
            asp,
            "stat({}, {}, {}).",
            stat.name(),
            (stat.utility_per_point() * 100.0).round() as i32,
            stat.cap()
        )?;

        if let Some(gem_category) = gem_category {
            writeln!(
                asp,
                "stat_to_gem_category({}, {}).",
                stat.name(),
                gem_category
            )?;
        }

        if let Some(cap) = stat.cap_stat() {
            writeln!(asp, "stat_to_cap({}, {}).", stat.name(), cap.name())?;
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
        writeln!(asp, "class_skill_line({}, {}).", line.category(), line)?;
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

pub fn stat_baseline_atoms(template: &Template, items: &[Item]) -> Result<String> {
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

    for item in items.iter().filter(|i| equipped_item_ids.contains(&i.id)) {
        for bonus in &item.bonuses {
            for stat in resolved_bonus_stats(bonus.stat, template.class) {
                if let Some(baseline) = baselines.get_mut(&stat) {
                    *baseline += bonus.value;
                }
            }
        }
    }

    for (stat, baseline) in baselines {
        writeln!(asp, "stat_baseline({}, {}).", stat.name(), baseline)?;
    }

    Ok(asp)
}

pub fn gem_atoms(preferences: &HashMap<u16, Preference>) -> Result<String> {
    let mut asp = String::new();
    writeln!(asp, "% --- SC GEMS ---")?;

    let target_stats = preferences
        .iter()
        .filter(|(_, pref)| pref.weight > 0)
        .filter_map(|(stat_id, _)| Stat::from_repr(*stat_id))
        .collect::<HashSet<_>>();
    let gems = Gem::generate_for_targets(&target_stats);

    for gem in gems {
        writeln!(
            asp,
            "gem({}, {}, {}, {}, {}).",
            gem.id,
            gem.stat.name(),
            gem.tier,
            gem.value,
            (gem.ip_cost * 10.0) as u16
        )?;
    }

    Ok(asp)
}

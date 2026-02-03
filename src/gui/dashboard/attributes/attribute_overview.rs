use anyhow::anyhow;
use dioxus::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::app_state::AppState;
use crate::core::domain::{stat::Stat, stat_category::StatCategory};
use crate::gui::dashboard::attributes::attribute_section::AttributeSection;
use crate::gui::dashboard::attributes::stat_data::StatData;

#[derive(Debug, PartialEq)]
struct ComputedAttributes {
    stats: HashMap<Stat, StatData>,
    resists: HashMap<Stat, StatData>,
    skills: HashMap<Stat, StatData>,
    toa: HashMap<Stat, StatData>,
}

#[component]
pub fn AttributeOverview() -> Element {
    let app_state = use_context::<Signal<AppState>>();

    let calculation_result = use_memo(move || -> Result<ComputedAttributes, String> {
        let state = app_state.read();
        let template_guard = state
            .template
            .lock()
            .map_err(|e| format!("Failed to lock template mutex: {}", e))?;
        let template = template_guard
            .as_ref()
            .ok_or_else(|| "No template active")?;

        let mut stats_map = HashMap::new();
        let mut caps_map = HashMap::new();
        let mut resists_map = HashMap::new();
        let mut skills_map = HashMap::new();
        let mut toa_map = HashMap::new();

        let class_skills = template.class.skill_lines();
        let class_acuity_stat = template.class.acuity_stat();

        for stat in Stat::iter() {
            match stat.category() {
                StatCategory::PhysicalStats => {
                    stats_map.insert(stat, StatData::new(stat));
                }
                StatCategory::AcuityStats => {
                    if Some(stat) == class_acuity_stat {
                        stats_map.insert(stat, StatData::new(stat));
                    }
                }
                StatCategory::PhysicalStatCaps | StatCategory::AcuityStatCaps => {
                    caps_map.insert(stat, StatData::new(stat));
                }
                StatCategory::Resists => {
                    resists_map.insert(stat, StatData::new(stat));
                }
                _ if class_skills.contains(&stat) => {
                    skills_map.insert(stat, StatData::new(stat));
                }
                _ => {}
            }
        }

        for item in template.slots.values() {
            for bonus in &item.bonuses {
                match bonus.stat.category() {
                    StatCategory::PhysicalStats => {
                        if let Some(entry) = stats_map.get_mut(&bonus.stat) {
                            entry.value += bonus.value;
                        }
                    }
                    StatCategory::AcuityStats => {
                        let target = match bonus.stat {
                            Stat::Acuity => class_acuity_stat,
                            _ => Some(bonus.stat),
                        };
                        if let Some(entry) = target.and_then(|s| stats_map.get_mut(&s)) {
                            entry.value += bonus.value;
                        }
                    }
                    StatCategory::PhysicalStatCaps => {
                        if let Some(entry) = caps_map.get_mut(&bonus.stat) {
                            entry.value += bonus.value;
                        }
                    }
                    StatCategory::AcuityStatCaps => {
                        let target = match bonus.stat {
                            Stat::AcuityCap => class_acuity_stat.and_then(|s| s.cap_stat()),
                            _ => Some(bonus.stat),
                        };
                        
                        if let Some(entry) = target.and_then(|s| caps_map.get_mut(&s)) {
                            entry.value += bonus.value;
                        }
                    }
                    StatCategory::Resists => {
                        if let Some(entry) = resists_map.get_mut(&bonus.stat) {
                            entry.value += bonus.value;
                        }
                    }
                    _ if class_skills.contains(&bonus.stat) => {
                        if let Some(entry) = skills_map.get_mut(&bonus.stat) {
                            entry.value += bonus.value;
                        }
                    }
                    StatCategory::ToaBonuses => {
                        let entry = toa_map
                            .entry(bonus.stat)
                            .or_insert_with(|| StatData::new(bonus.stat));

                        entry.value += bonus.value;
                    }
                    _ => {}
                }
            }
        }

        for (cap_stat, cap_data) in &caps_map {
            if let Some(base_stat) = cap_stat.base_stat() {
                let cap_bonus = cap_data.value.min(cap_stat.cap());
                if let Some(base_entry) = stats_map.get_mut(&base_stat) {
                    base_entry.cap += cap_bonus;
                }
                // TODO: caps for other stuff
            }
        }

        Ok(ComputedAttributes {
            stats: stats_map,
            resists: resists_map,
            skills: skills_map,
            toa: toa_map,
        })
    });

    let result = calculation_result.read();
    let attributes = result.as_ref().map_err(|e| anyhow!("{}", e))?;

    let mut base: Vec<StatData> = attributes.stats.values().cloned().collect();
    base.sort_by_key(|d| d.stat);

    let mut resists: Vec<StatData> = attributes.resists.values().cloned().collect();
    resists.sort_by_key(|d| d.stat);

    let mut skills: Vec<StatData> = attributes.skills.values().cloned().collect();
    skills.sort_by_key(|d| d.stat);

    let mut toa: Vec<StatData> = attributes.toa.values().cloned().collect();
    toa.sort_by_key(|d| d.stat);

    rsx! {
        div { class: "w-full max-w-[1600px] mx-auto pb-10 px-4",

            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4 items-start",

                AttributeSection { title: "Stats", stats: base }
                AttributeSection { title: "Resists", stats: resists }
                AttributeSection { title: "Skills", stats: skills }
                AttributeSection { title: "Bonuses", stats: toa }
            }
        }
    }
}

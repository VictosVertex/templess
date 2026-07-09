use serde::Deserialize;

use crate::core::domain::{
    class::Class, preference::Preference, preference_preset::PreferencePreset, stat::Stat 
};

#[derive(Debug, Clone, Deserialize)]
pub struct RawPreferencePreset {
    pub name: String,
    pub class_id: u16,
    pub preferences: Vec<RawPresetPreference>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawPresetPreference {
    pub stat_id: u16,
    pub min: u16,
    pub weight: u16,
}

impl RawPreferencePreset {
    pub fn into_preset(self, id: u32) -> Option<PreferencePreset> {
        let class = Class::from_repr(self.class_id)?;

        let mut preferences = std::collections::HashMap::new();

        for raw_pref in self.preferences {
            if let Some(stat) = Stat::from_repr(raw_pref.stat_id) {
                preferences.insert(
                    stat,
                    Preference {
                        min: raw_pref.min,
                        weight: raw_pref.weight,
                    },
                );
            }
        }

        Some(PreferencePreset::new(id, class, self.name, preferences))
    }
}
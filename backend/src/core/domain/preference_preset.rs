use std::collections::HashMap;
use crate::core::domain::{class::Class, preference::Preference, stat::Stat};

#[derive(Debug, Clone)]
pub struct PreferencePreset {
    id: u32,
    class: Class,
    name: String,
    preferences: HashMap<Stat, Preference>,
}

impl PreferencePreset {
    pub fn new(id: u32, class: Class, name: String, preferences: HashMap<Stat, Preference>) -> Self {
        Self { id, class, name, preferences }
    }

    pub fn id(&self) -> u32 { self.id }
    pub fn class(&self) -> Class { self.class }
    pub fn name(&self) -> &str { &self.name }
    pub fn preferences(&self) -> &HashMap<Stat, Preference> { &self.preferences }
}
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub class_id: u16,
    pub preference_preset_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct PreferenceRequest {
    pub min: u16,
    pub weight: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EquipSourceRequest {
    User,
    Optimizer,
}

#[derive(Deserialize)]
pub struct EquippedItemStateRequest {
    pub item_id: u32,
    pub source: EquipSourceRequest,
    pub gem_ids: Vec<u32>,
}

#[derive(Deserialize)]
pub struct UpdateTemplateRequest {
    pub preferences: HashMap<u16, PreferenceRequest>,
    pub equipped_items: HashMap<u16, EquippedItemStateRequest>,
}

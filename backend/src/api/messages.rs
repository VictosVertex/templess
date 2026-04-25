use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::domain::{
    class::Class, item_slot::ItemSlot, preference::Preference, template::Template,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ClientMessage {
    Start(OptimizationRequest),
    Cancel,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PreferenceDTO {
    pub min: u16,
    pub weight: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OptimizationRequest {
    pub class_id: u16,
    pub equipped_items: HashMap<u16, u32>,
    pub preferences: HashMap<u16, PreferenceDTO>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ServerMessage {
    Setup,

    Grounding,

    Solving,

    NewModel {
        optimized_items: HashMap<u16, u32>,
        slotted_gems: HashMap<u32, Vec<u32>>,
    },

    Finished,

    Canceled,

    Error { message: String },
}

impl TryFrom<OptimizationRequest> for Template {
    type Error = String;

    fn try_from(req: OptimizationRequest) -> Result<Self, Self::Error> {
        let class = Class::from_repr(req.class_id)
            .ok_or_else(|| format!("Invalid class ID: {}", req.class_id))?;

        let slots = req
            .equipped_items
            .iter()
            .filter_map(|(slot_id, item_id)| {
                ItemSlot::from_repr(*slot_id).map(|slot| (slot, *item_id as i32))
            })
            .collect();

        let preferences = req
            .preferences
            .iter()
            .filter_map(|(stat_id, preference)| {
                Some((
                    *stat_id,
                    Preference {
                        min: preference.min,
                        weight: preference.weight,
                    },
                ))
            })
            .collect();

        Ok(Template {
            id: 0,
            name: "optimization".to_string(),
            class,
            slots,
            preferences,
        })
    }
}

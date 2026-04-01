use std::collections::HashMap;

use crate::core::domain::{
    class::Class, item::Item, item_slot::ItemSlot, item_type::ItemType, realm::Realm, stat::Stat,
    template::Template,
};

#[derive(serde::Serialize)]
pub struct RealmResponse {
    pub id: u16,
    pub name: String,
}

impl From<Realm> for RealmResponse {
    fn from(realm: Realm) -> Self {
        RealmResponse {
            id: realm.id(),
            name: realm.to_string(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ClassResponse {
    pub id: u16,
    pub name: String,
    pub realm_id: u16,
    pub acuity_stat_id: u16,
    pub skill_line_ids: Vec<u16>,
    pub allowed_item_type_ids: Vec<u16>,
}

impl From<Class> for ClassResponse {
    fn from(class: Class) -> Self {
        ClassResponse {
            id: class.id(),
            name: class.to_string(),
            realm_id: class.realm().id(),
            acuity_stat_id: class.acuity_stat().map_or(0, |stat| stat.id()),
            skill_line_ids: class.skill_lines().iter().map(|line| line.id()).collect(),
            allowed_item_type_ids: class
                .allowed_item_types()
                .iter()
                .map(|item_type| item_type.id())
                .collect(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ItemResponse {
    /// The unique identifier for the item.
    pub id: i32,

    /// The name of the item.
    pub name: String,

    /// The appearance model of the item.
    pub model: i32,

    /// The type of object.
    pub object_type_id: u16,

    /// The slot of the item.
    pub item_slot_id: u16,

    /// The hand the weapon is wielded in.
    pub weapon_hand: u16,

    /// The single value utility of the item.
    pub utility_single: f32,

    /// The total utility of the item.
    pub utility: f32,

    /// The ids of the bonuses applied to the item.
    pub bonus_ids: HashMap<u16, u16>,
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        ItemResponse {
            id: item.id,
            name: item.name,
            model: item.model,
            object_type_id: item.object_type.id(),
            item_slot_id: item.item_slot.id(),
            weapon_hand: item.weapon_hand,
            utility_single: item.utility_single,
            utility: item.utility,
            bonus_ids: item
                .bonuses
                .iter()
                .map(|bonus| (bonus.stat.id(), bonus.value))
                .collect(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct StatResponse {
    pub id: u16,
    pub name: String,
}

impl From<Stat> for StatResponse {
    fn from(stat: Stat) -> Self {
        StatResponse {
            id: stat.id(),
            name: stat.to_string(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ItemTypeResponse {
    pub id: u16,
    pub name: String,
}

impl From<ItemType> for ItemTypeResponse {
    fn from(item_type: ItemType) -> Self {
        ItemTypeResponse {
            id: item_type.id(),
            name: item_type.to_string(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ItemSlotResponse {
    pub id: u16,
    pub name: String,
}

impl From<ItemSlot> for ItemSlotResponse {
    fn from(item_slot: ItemSlot) -> Self {
        ItemSlotResponse {
            id: item_slot.id(),
            name: item_slot.name(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct TemplatesResponse {
    pub id: i32,
    pub name: String,
    pub class_id: u16,
    pub slots: HashMap<u16, i32>,
}

impl From<Template> for TemplatesResponse {
    fn from(template: Template) -> Self {
        TemplatesResponse {
            id: template.id,
            name: template.name,
            class_id: template.class.id(),
            slots: template
                .slots
                .iter()
                .map(|(slot, item_id)| (slot.id(), *item_id))
                .collect(),
        }
    }
}

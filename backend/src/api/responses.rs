use std::collections::HashMap;

use crate::core::domain::{
    class::Class, equip::EquippedItemState, gem::Gem, item::Item, item_slot::ItemSlot, item_type::ItemType, preference::Preference, preference_preset::PreferencePreset, realm::Realm, stat::Stat, template::Template
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
    pub id: u32,

    /// The name of the item.
    pub name: String,

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

    /// The ids of the bonuses applied to the item and their corresponding values.
    pub bonuses: HashMap<u16, u16>,

    /// Whether the item is a dropped item or a craft base.
    pub source: crate::core::domain::item::ItemSource,

    /// The price of the item in the specified currency.
    pub price: u32,

    /// The currency of the price
    pub currency: u8,

    /// Human readable label for the price currency.
    pub currency_label: String,
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        let currency_label = format!("{:?}", item.currency)
            .replace("SummonersHall", "Summoner's Hall")
            .replace("TrialsOfAtlantis", "Trials of Atlantis")
            .replace("DarknessFalls", "Darkness Falls")
            .replace("BountyPoints", "Bounty Points");

        ItemResponse {
            id: item.id,
            name: item.name,
            object_type_id: item.object_type.id(),
            item_slot_id: item.item_slot.id(),
            weapon_hand: item.weapon_hand,
            utility_single: item.utility_single,
            utility: item.utility,
            bonuses: item
                .bonuses
                .iter()
                .map(|bonus| (bonus.stat.id(), bonus.value))
                .collect(),
            source: item.source,
            price: item.price,
            currency: item.currency as u8,
            currency_label,
        }
    }
}

#[derive(serde::Serialize)]
pub struct StatResponse {
    pub id: u16,
    pub name: String,
    pub utility: f32,
    pub cap: u16,
    pub category_id: u16,
    pub base_stat_id: Option<u16>,
}

impl From<Stat> for StatResponse {
    fn from(stat: Stat) -> Self {
        StatResponse {
            id: stat.id(),
            name: stat.to_string(),
            cap: stat.cap(),
            utility: stat.utility_per_point(),
            category_id: stat.category().id(),
            base_stat_id: stat.base_stat().map(|s| s.id()),
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct GemResponse {
    pub id: u32,
    pub stat_id: u16,
    pub tier: u16,
    pub value: u16,
    pub ip_cost: f32,
}

impl From<Gem> for GemResponse {
    fn from(gem: Gem) -> Self {
        GemResponse {
            id: gem.id,
            stat_id: gem.stat.id(),
            tier: gem.tier,
            value: gem.value,
            ip_cost: gem.ip_cost,
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
#[serde(rename_all = "camelCase")]
pub enum EquipSourceResponse {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "optimizer")]
    Optimizer,
}

#[derive(serde::Serialize)]
pub struct EquippedItemStateResponse {
    pub item_id: u32,
    pub source: EquipSourceResponse,
    pub gem_ids: Vec<u32>,
}

impl From<EquippedItemState> for EquippedItemStateResponse {
    fn from(equip: EquippedItemState) -> Self {
        EquippedItemStateResponse {
            item_id: equip.item_id,
            source: match equip.source {
                crate::core::domain::equip::EquipSource::User => EquipSourceResponse::User,
                crate::core::domain::equip::EquipSource::Optimizer => {
                    EquipSourceResponse::Optimizer
                }
            },
            gem_ids: equip.gem_ids,
        }
    }
}

#[derive(serde::Serialize)]
pub struct PreferencePresetResponse {
    pub id: u32,
    pub class_id: u16,
    pub name: String,
    pub preferences: HashMap<u16, PreferenceResponse>,
}

impl From<PreferencePreset> for PreferencePresetResponse {
    fn from(preset: PreferencePreset) -> Self {
        Self {
            id: preset.id(),
            class_id: preset.class().id(),
            name: preset.name().to_string(),
            preferences: preset
                .preferences()
                .iter()
                .map(|(stat, pref)| (stat.id(), pref.clone().into()))
                .collect(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct PreferenceResponse {
    pub min: u16,
    pub weight: u16,
}

impl From<Preference> for PreferenceResponse {
    fn from(pref: Preference) -> Self {
        PreferenceResponse {
            min: pref.min as u16,
            weight: pref.weight as u16,
        }
    }
}

#[derive(serde::Serialize)]
pub struct TemplatesResponse {
    pub id: i32,
    pub name: String,
    pub class_id: u16,
    pub equipped_items: HashMap<u16, EquippedItemStateResponse>,
    pub preferences: HashMap<u16, PreferenceResponse>,
}

impl From<Template> for TemplatesResponse {
    fn from(template: Template) -> Self {
        TemplatesResponse {
            id: template.id,
            name: template.name,
            class_id: template.class.id(),
            equipped_items: template
                .equipped_items
                .into_iter()
                .map(|(slot, equip)| (slot.id(), equip.into()))
                .collect(),
            preferences: template
                .preferences
                .into_iter()
                .map(|(stat_id, pref)| (stat_id, pref.into()))
                .collect(),
        }
    }
}

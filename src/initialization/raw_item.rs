//! This module defines the `RawItem` struct and its conversion to the `Item` domain model.
//!
//! `RawItems` are items as they are found in the json data scraped from the game server's website.

use crate::core::domain::{
    class::Class, item::Item, item_bonus::ItemBonus, item_slot::ItemSlot, item_type::ItemType,
    realm::Realm, stat::Stat,
};

use serde::Deserialize;

/// Represents a raw item as it is found in the JSON data scraped from the game server's website.
///
/// This struct is used to deserialize the raw item data and convert it into the application's `Item` type.
#[derive(Debug, Clone, Deserialize)]
pub struct RawItem {
    /// The appearance model of the item/
    pub model: String,

    /// The unique identifier for the item.
    pub id: String,

    /// The type of object.
    pub object_type: String,

    /// The type of the item, represented as a string that can be parsed into an `ItemType`.
    pub item_type: String,

    /// Level of the item itself.
    pub level: String,

    /// Quality of the item.
    pub quality: String,

    /// The hand the weapon is wielded in.
    pub weapon_hand: String,

    /// The speed of the weapon.
    pub weapon_speed: String,

    /// The type of damage the item deals.
    pub damage_type: String,

    /// The realm the item belongs to, represented as a string that can be parsed into a `Realm`.
    pub realm: String,

    /// The required level to use the item.
    pub required_level: u16,

    /// The bonus level of the item.
    pub bonus_level: String,

    /// The size of the shield, if applicable.
    pub shield_size: String,

    /// The type of instrument, if applicable.
    pub instrument_type: String,

    /// Indicates if the item is tradable.
    pub is_tradable: String,

    /// The name of the item.
    pub name: String,

    /// The single value utility of the item.
    pub utility_single: String,

    /// The total utility of the item.
    pub utility: String,

    /// The classes allowed to use this item.
    ///
    /// This is a semicolon-separated list of class IDs.
    pub allowed_classes: String,

    /// The types of bonuses applied to the item.
    ///
    /// This is a comma-separated list of bonus type IDs.
    pub bonus_types: String,

    /// The values of the bonuses applied to the item.
    ///
    /// This is a comma-separated list of bonus values corresponding to the `bonus_types`.
    pub bonus_values: String,

    /// JSON representations of the first proc.
    pub proc1_json: Option<String>,

    /// JSON representations of the second proc.
    pub proc2_json: Option<String>,

    /// JSON representations of the first use effect.
    pub use1_json: Option<String>,

    /// JSON representations of the second use effect.
    pub use2_json: Option<String>,

    /// JSON representations of passive effects.
    pub passive_json: Option<String>,

    /// JSON representations of the first reactive effect.
    pub react1_json: Option<String>,

    /// JSON representations of the second reactive effect.
    pub react2_json: Option<String>,
}

impl RawItem {
    /// Converts the `RawItem` into an `Item`, applying necessary transformations and validations.
    ///
    /// # Returns
    /// An `Option<Item>`, which will be `Some(Item)` if the conversion is successful,
    /// or `None` if any of the required fields are invalid or cannot be parsed.
    pub fn into_item(self) -> Option<Item> {
        let allowed_classes = self
            .allowed_classes
            .split(';')
            .filter_map(|c| c.parse::<u16>().ok())
            .filter_map(Class::from_repr)
            .collect::<Vec<Class>>();

        let bonuses = self
            .bonus_types
            .split(',')
            .zip(self.bonus_values.split(','))
            .filter_map(|(t, v)| match (t.parse::<u16>(), v.parse::<u16>()) {
                (Ok(t), Ok(v)) => Some((t, v)),
                _ => None,
            })
            .filter_map(|(t, v)| Stat::from_repr(t).map(|stat| ItemBonus { stat, value: v }))
            .collect::<Vec<ItemBonus>>();

        let id = self.id.parse().expect("Failed to parse id");
        let model = self.model.parse().expect("Failed to parse model");
        let object_type_id = self
            .object_type
            .parse()
            .expect("Failed to parse object_type");
        let object_type = ItemType::from_repr(object_type_id).expect("Invalid object_type repr");
        let item_slot_id = self
            .item_type
            .parse::<u16>()
            .expect("Failed to parse item_type");
        let item_slot = ItemSlot::from_repr(item_slot_id).expect("Invalid item_type repr");
        let level = self.level.parse().expect("Failed to parse level");
        let quality = self.quality.parse().expect("Failed to parse quality");
        let weapon_hand = self
            .weapon_hand
            .parse()
            .expect("Failed to parse weapon_hand");
        let weapon_speed = self
            .weapon_speed
            .parse()
            .expect("Failed to parse weapon_speed");
        let damage_type = self
            .damage_type
            .parse()
            .expect("Failed to parse damage_type");
        let realm_id = self.realm.parse::<u16>().expect("Failed to parse realm");
        let realm = Realm::from_repr(realm_id).expect("Invalid realm repr");
        let bonus_level = self
            .bonus_level
            .parse()
            .expect("Failed to parse bonus_level");
        let shield_size = self
            .shield_size
            .parse()
            .expect("Failed to parse shield_size");
        let instrument_type = self
            .instrument_type
            .parse()
            .expect("Failed to parse instrument_type");
        let is_tradable = self
            .is_tradable
            .parse::<u16>()
            .expect("Failed to parse is_tradable")
            > 0;
        let utility_single = self
            .utility_single
            .parse()
            .expect("Failed to parse utility_single");
        let utility = self.utility.parse().expect("Failed to parse utility");

        Some(Item {
            id,
            name: self.name.clone(),
            model,
            object_type,
            item_slot,
            level,
            quality,
            realm,
            weapon_hand,
            weapon_speed,
            damage_type,
            required_level: self.required_level,
            bonus_level,
            shield_size,
            instrument_type,
            is_tradable,
            utility_single,
            utility,
            allowed_classes,
            bonuses,
            proc1_json: self.proc1_json.clone(),
            proc2_json: self.proc2_json.clone(),
            use1_json: self.use1_json.clone(),
            use2_json: self.use2_json.clone(),
            passive_json: self.passive_json.clone(),
            react1_json: self.react1_json.clone(),
            react2_json: self.react2_json.clone(),
        })
    }
}

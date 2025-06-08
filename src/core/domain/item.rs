//! This module defines the `Item` entity and its properties.
//!
//! Since this application deals with templates, an optimized set of items, in Dark Age of Camelot (DAoC),
//! the `Item` struct is the core unit of this program.
//!
//! As that it is the most likely to be changed as requirements evolve.

use super::{class::Class, item_bonus::ItemBonus, item_type::ItemType, realm::Realm};

/// Represents an item in the game.
///
/// This struct contains all the properties of an item after it has been parsed and validated
/// from raw data sources such as JSON.
#[derive(Debug, PartialEq)]
pub struct Item {
    /// The unique identifier for the item.
    pub id: i32,

    /// The name of the item.
    pub name: String,

    /// The appearance model of the item.
    pub model: i32,

    /// The type of object.
    pub object_type: u16,

    /// The type of the item.
    pub item_type: ItemType,

    /// The level of the item.
    pub level: u16,

    /// The quality of the item.
    pub quality: u16,

    /// The hand the weapon is wielded in.
    pub weapon_hand: u16,

    /// The speed of the weapon.
    pub weapon_speed: u16,

    /// The type of damage the item deals.
    pub damage_type: u16,

    /// The realm the item belongs to.
    pub realm: Realm,

    /// The required level to use the item.
    pub required_level: u16,

    /// The bonus level of the item.
    pub bonus_level: u16,

    /// The size of the shield, if applicable.
    pub shield_size: u16,

    /// The type of instrument, if applicable.
    pub instrument_type: u16,

    /// Indicates if the item is tradable.
    pub is_tradable: bool,

    /// The single value utility of the item.
    pub utility_single: f32,

    /// The total utility of the item.
    pub utility: f32,

    /// The classes allowed to use this item.
    pub allowed_classes: Vec<Class>,

    /// The bonuses applied to the item.
    pub bonuses: Vec<ItemBonus>,

    /// JSON representation of the first proc effect, if any.
    pub proc1_json: Option<String>,

    /// JSON representation of the second proc effect, if any.
    pub proc2_json: Option<String>,

    /// JSON representation of the first use effect, if any.
    pub use1_json: Option<String>,

    /// JSON representation of the second use effect, if any.
    pub use2_json: Option<String>,

    /// JSON representation of passive effects, if any.
    pub passive_json: Option<String>,

    /// JSON representation of the first reactive effect, if any.
    pub react1_json: Option<String>,

    /// JSON representation of the second reactive effect, if any.
    pub react2_json: Option<String>,
}

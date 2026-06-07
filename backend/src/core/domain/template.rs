//! This moldule defines the template entity.

use std::collections::HashMap;

use crate::core::domain::{
    class::Class, equip::EquippedItemState, item_slot::ItemSlot, preference::Preference,
};

/// Represents a Dark Age of Camelot template.
///
/// A template is a set of items associated with a character class.
/// Each slot is supposed to hold a specific item such that the preferred
/// stats of the character are as close to their cap as possible.
///
/// This represents precisely the optimization problem this application
/// is trying to solve.
#[derive(Debug, Clone)]
pub struct Template {
    pub id: i32,
    /// The name of the template.
    pub name: String,

    /// The class associated with the template.
    pub class: Class,

    /// The equipped items
    pub equipped_items: HashMap<ItemSlot, EquippedItemState>,

    pub preferences: HashMap<u16, Preference>,
}

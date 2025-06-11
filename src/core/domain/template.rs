//! This moldule defines the template entity.

use std::{collections::HashMap, sync::Arc};

use crate::core::domain::{class::Class, item::Item, item_slot::ItemSlot};

/// Represents a Dark Age of Camelot template.
/// 
/// A template is a set of items associated with a character class.
/// Each slot is supposed to hold a specific item such that the preferred
/// stats of the character are as close to their cap as possible.
/// 
/// This represents precisely the optimization problem this application
/// is trying to solve.
#[derive(Debug)]
pub struct Template {
    /// The name of the template.
    pub name: String,

    /// The class associated with the template.
    pub class: Class,

    /// The slots and their associated items.
    pub slots: HashMap<ItemSlot, Arc<Item>>
}


impl Template {
    /// Creates a new template with the given name and class.
    ///
    /// # Arguments
    /// * `name` - The name of the template.
    /// * `class` - The class associated with the template.
    pub fn new(class: Class) -> Self {
        Self {
            name: "Untitled Template".to_string(),
            class,
            slots: HashMap::new(),
        }
    }

    /// Set a specific item in a given slot.
    ///
    /// # Arguments
    /// * `slot` - The item slot where the item should be placed.
    /// * `item` - The item to be placed in the slot.
    pub fn set_item(&mut self, slot: ItemSlot, item: Arc<Item>) {
        self.slots.insert(slot, item);
    }

    /// Get the item in a specific slot.
    ///
    /// # Arguments
    /// * `slot` - The item slot to retrieve the item from.
    ///
    /// # Returns
    /// An `Option` containing a reference to the item if it exists, or `None` if the slot is empty.
    pub fn get_item(&self, slot: &ItemSlot) -> Option<&Arc<Item>> {
        self.slots.get(slot)
    }
}
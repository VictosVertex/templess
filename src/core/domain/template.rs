//! This moldule defines the template entity.

use std::collections::HashMap;

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
    pub slots: HashMap<ItemSlot, Item>
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
}
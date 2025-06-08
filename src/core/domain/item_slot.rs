//! This module defines the `ItemSlot` enum, which represents various item slots in a game.

use std::fmt::Display;

use strum::FromRepr;

/// Represents the different item slots available in the game.
///
/// Each variant corresponds to a specific item slot, such as head, hands, feet, etc.
///
/// Since some items can be equipped in multiple slots (like rings and bracers),
/// the enum contains `Ring2` and `Bracer2` to represent the second instance of these slots.
/// These do not exist in the game database, but may simplify logic in the codebase.
#[repr(u16)]
#[derive(Debug, Clone, Copy, FromRepr, PartialEq)]
pub enum ItemSlot {
    /// Represents the head slot.
    Head = 21,

    /// Represents the hands slot.
    Hands = 22,

    /// Represents the feet slot.
    Feet = 23,

    /// Represents the chest slot.
    Chest = 25,

    /// Represents the legs slot.
    Legs = 27,

    /// Represents the arms slot.
    Arms = 28,

    /// Represents the jewel slot.
    Jewel = 24,

    /// Represents the cloak slot.
    Cloak = 26,

    /// Represents the necklace slot.
    Necklace = 29,

    /// Represents the belt slot.
    Belt = 32,

    /// Represents the bracer slot.
    ///
    /// This is the correct ID for any bracer item.
    Bracer = 33,

    /// Represents the second bracer slot.
    ///
    /// This is not the correct ID, the correct ID is 33 but a character can hold two bracers.
    Bracer2 = 34,

    /// Represents the ring slot.
    ///
    /// This is the correct ID for any ring item.
    Ring = 35,

    /// Represents the second ring slot.
    ///
    /// This is not the correct ID, the correct ID is 35 but a character can hold two rings.
    Ring2 = 36,

    /// Represents the right hand slot.
    RightHand = 10,

    /// Represents the left hand slot.
    LeftHand = 11,

    /// Represents the two-handed weapon slot.
    TwoHanded = 12,

    /// Represents the ranged weapon slot.
    Ranged = 13,
}

impl ItemSlot {
    /// Returns the ID of the item slot as an `i32`.
    ///
    /// This is useful for database operations or when an integer representation is needed.
    pub fn id(&self) -> i32 {
        *self as i32
    }
}

impl Display for ItemSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemSlot::Ring2 => write!(f, "Ring"),
            ItemSlot::Bracer2 => write!(f, "Bracer"),
            _ => write!(f, "{:?}", self),
        }
    }
}
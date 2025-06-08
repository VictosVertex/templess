//! This module defines the `ItemType` enum, which represents various types of items in the game.

use strum::FromRepr;

/// Represents the different item types available in the game.
///
/// Each variant corresponds to a specific type of item, such as crush, slash, thrust, etc.
#[repr(u16)]
#[derive(Debug, Clone, Copy, FromRepr, PartialEq)]
pub enum ItemSlot {
    /// Represents the crush melee weapon type from Albion.
    Crush = 2,

    /// Represents the slash melee weapon type from Albion.
    Slash = 3,

    /// Represents the thrust melee weapon type from Albion.
    Thrust = 4,

    /// Represents the short bow ranged weapon used in Albion and Hibernia.
    ShortBow = 5,

    /// Represents the two-handed melee weapon type from Albion.
    TwoHanded = 6,

    /// Represents the polearm melee weapon type from Albion.
    Polearm = 7,

    /// Represents the staff casting weapon type.
    ///
    /// This is used as a casting weapon for classes like Theurgist and Spiritmaster.
    /// Additionally, it is used by the Albion Friar class as a melee weapon.
    Staff = 8,

    /// Represents the long bow ranged weapon type used by Scouts.
    LongBow = 9,

    /// Represents the crossbow ranged weapon type from Albion.
    Crossbow = 10,

    /// Represents the sword melee weapon type from Midgard.
    Sword = 11,

    /// Represents the hammer melee weapon type from Midgard.
    Hammer = 12,

    /// Represents the axe melee weapon type from Midgard.
    Axe = 13,

    /// Represents the spear melee weapon type from Midgard.
    Spear = 14,

    /// Represents the composite bow ranged weapon type used by Hunters.
    CompositeBow = 15,

    /// Represents the left axe melee weapon type from Midgard.
    LeftAxe = 17,

    /// Represents the recurve bow ranged weapon type from Hibernia.
    RecurveBow = 18,

    /// Represents the blade melee weapon type from Hibernia.
    Blade = 19,

    /// Represents the blunt melee weapon type from Hibernia.
    Blunt = 20,

    /// Represents the piercing melee weapon type from Hibernia.
    Piercing = 21,

    /// Represents the large weapon type from Hibernia.
    LargeWeapon = 22,

    /// Represents the Celtic spear melee weapon type from Hibernia.
    CelticSpear = 23,

    /// Represents the flexible melee weapon type from Albion.
    Flexible = 24,

    /// Represents the hand-to-hand melee weapon type used by Savages.
    HandToHand = 25,

    /// Represents the scythe melee weapon type used by Valewalkers.
    Scythe = 26,

    /// Represents the mauler fist wrap melee weapon type used by Maulers.
    MaulerFistWrap = 27,

    /// Represents the mauler staff melee weapon type used by Maulers.
    MaulerStaff = 28,

    /// Represents the magical item type, which is used for jewelry such as rings and necklaces.
    Magical = 29,

    /// Represents the cloth armor type.
    Cloth = 32,

    /// Represents the leather armor type.
    Leather = 33,

    /// Represents the studded armor type.
    Studded = 34,

    /// Represents the chain armor type.
    Chain = 35,

    /// Represents the plate armor type.
    Plate = 36,

    /// Represents the reinforced armor type.
    Reinforced = 37,

    /// Represents the scale armor type.
    Scale = 38,

    /// Represents the jewelry item type.
    Jewelry = 41,

    /// Represents the shield item type.
    Shield = 42,

    /// Represents the instrument item type.
    ///
    /// This is not the correct ID, but it is used in the game
    Instrument = 45,

    /// Represents an unknown item type.
    Unknown = 1000, // Placeholder for unknown item types
}

impl ItemSlot {
    /// Converts the `ItemType` to its corresponding numeric ID.
    ///
    /// # Returns
    /// The numeric ID of the item type as an `u16`.
    pub fn id(&self) -> u16 {
        *self as u16
    }
}

//! This file defines the `StatCategory` enum, which represents different categories of stats in the game.
//!
//! It was introduced because the increase to some stats may result in an increase to other stats.
//! For example, increasing acuity also increases intelligence if the character is a caster with intelligence as a primary stat.

use strum::FromRepr;

/// Represents the different categories of stats in the game.
///
/// Each variant corresponds to a specific category of stats, such as general stats,
/// acuity stats, stat caps, and various skill categories.
#[repr(u16)]
#[derive(Debug, Clone, Copy, FromRepr)]
pub enum StatCategory {
    /// Represents general stats like strength, dexterity, etc.
    PhysicalStats = 0,

    /// Represents acuity stats, which are specific to caster classes.
    AcuityStats = 1,

    /// Represents stat cap increases for regular stats.
    PhysicalStatCaps = 2,

    /// Represents stat cap increases for acuity stats.
    AcuityStatCaps = 3,

    /// Represents resistances, such as heat, cold, body, etc.
    Resists = 4,

    /// Represents magic skills, such as Regrowth.
    MagicSkills = 5,

    /// Represents melee skills, such as Large Weapons.
    MeleeSkills = 6,

    /// Represents archery lines, such as Long Bow.
    ArcherySkills = 7,

    /// Represents dual wielding skills, such as Left Axe.
    DualWieldingSkills = 8,

    /// Represents other skills, such as Stealth.
    OtherSkills = 9,

    /// Represents Trials of Atlantis bonuses, such as spell duration.
    ToaBonuses = 10,

    /// Represents other stats that do not fit into the above categories.
    OtherStats = 11,
}

impl StatCategory {
    /// Returns the numeric identifier of the stat category.
    pub fn id(&self) -> u16 {
        *self as u16
    }
}

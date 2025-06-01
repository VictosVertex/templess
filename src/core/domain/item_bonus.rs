//! This module defines the item bonus structure used in the game.
//!
//! An `ItemBonus` is a specific amount of a particular stat applied to an item.
use super::stat::Stat;

/// Represents an item bonus in the game.
///
/// Specifies how much of a specific stat is applied to an item.
#[derive(Debug)]
pub struct ItemBonus {
    /// The stat of this bonus.
    ///
    /// This is the specific type of stat like strength, dexterity, etc.
    pub stat: Stat,

    /// The value of the bonus.
    ///
    /// This is the amount of the stat that is applied to the item.
    pub value: u16,
}

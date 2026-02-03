//! This module defines the `StatData` struct,
//! which represents the data for a specific stat,
//! including its current value and cap.

use crate::core::domain::stat::Stat;

/// Represents the data for a specific stat.
#[derive(Clone, PartialEq, Debug)]
pub struct StatData {
    /// The stat this data represents.
    pub stat: Stat,
    /// The current value of the stat.
    pub value: u16,
    /// The cap of the stat.
    pub cap: u16,
}

impl StatData {
    /// Creates a new `StatData` instance for the given stat with an initial value of 0.
    ///
    /// # Parameters
    /// - `stat`: The stat to create data for.
    ///
    /// # Returns
    /// A new `StatData` instance.
    pub fn new(stat: Stat) -> Self {
        Self {
            stat,
            value: 0,
            cap: stat.cap(),
        }
    }
}

//! This module defines the `Realm` enum, representing the realms in the game.

use strum::{Display, EnumIter, EnumString, FromRepr};

/// Represents the realms in the game.
///
/// The `Realm` enum defines the four realms: All, Albion, Midgard, and Hibernia.
/// Each variant corresponds to a specific realm, with `All` representing all realms combined.
#[repr(u16)]
#[derive(Debug, Clone, Copy, EnumString, FromRepr, PartialEq, EnumIter, Display)]
pub enum Realm {
    /// Represents all realms combined.
    All,

    /// Represents the Albion realm.
    ///
    /// This is the first realm in the game, known for its knights and magic.
    Albion,

    /// Represents the Midgard realm.
    ///
    /// This is the second realm in the game, known for its warriors and shamans.
    Midgard,

    /// Represents the Hibernia realm.
    ///
    /// This is the third realm in the game, known for its druids and nature magic.
    Hibernia,
}

impl Realm {
    /// Returns the ID of the realm.
    pub fn id(&self) -> u16 {
        *self as u16
    }
}

impl From<Realm> for u16 {
    fn from(realm: Realm) -> Self {
        realm as u16
    }
}

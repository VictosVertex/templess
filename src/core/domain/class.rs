//! This module defines the `Class` enum and its associated functionality.
//!
//! It represents a character class in Dark Age of Camelot,
//! including its numeric identifier, name, and associated realm.

use strum::FromRepr;

use crate::core::domain::realm::Realm;

/// Representation of a character class in Dark Age of Camelot.
///
///
/// # Variants
///
/// - Albion: Paladin, Armsman, Scout, Minstrel, Theurgist, Cleric, Wizard, Sorcerer, Infiltrator,
///   Friar, Mercenary, Necromancer, Cabalist, Reaver, Heretic
/// - Midgard: Thane, Warrior, Shadowblade, Skald, Hunter, Healer, Spiritmaster, Shaman, Runemaster,
///   Bonedancer, Berserker, Savage, Valkyrie, Warlock
/// - Hibernia: Eldritch, Enchanter, Mentalist, Blademaster, Hero, Champion, Warden, Druid, Bard,
///   Nightshade, Ranger, Animist, Valewalker, Banshee
///
/// # Methods
///
/// - [`id`](#method.id): Returns the numeric identifier of the class.
/// - [`name`](#method.name): Returns the display name of the class.
/// - [`realm`](#method.realm): Returns the associated realm of the class.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromRepr)]
pub enum Class {
    /// Paladin class from Albion.
    Paladin = 1,

    /// Armsman class from Albion.
    Armsman = 2,

    /// Scout class from Albion.
    Scout = 3,

    /// Minstrel class from Albion.
    Minstrel = 4,

    /// Theurgist class from Albion.
    Theurgist = 5,

    /// Cleric class from Albion.
    Cleric = 6,

    /// Wizard class from Albion.
    Wizard = 7,

    /// Sorcerer class from Albion.
    Sorcerer = 8,

    /// Infiltrator class from Albion.
    Infiltrator = 9,

    /// Friar class from Albion.
    Friar = 10,

    /// Mercenary class from Albion.
    Mercenary = 11,

    /// Necromancer class from Albion.
    Necromancer = 12,

    /// Cabalist class from Albion.
    Cabalist = 13,

    /// Reaver class from Albion.
    Reaver = 19,

    /// Heretic class from Albion.
    Heretic = 33,

    /// Thane class from Midgard.
    Thane = 21,

    /// Warrior class from Midgard.
    Warrior = 22,

    /// Shadowblade class from Midgard.
    Shadowblade = 23,

    /// Skald class from Midgard.
    Skald = 24,

    /// Hunter class from Midgard.
    Hunter = 25,

    /// Healer class from Midgard.
    Healer = 26,

    /// Spiritmaster class from Midgard.
    Spiritmaster = 27,

    /// Shaman class from Midgard.
    Shaman = 28,

    /// Runemaster class from Midgard.
    Runemaster = 29,

    /// Bonedancer class from Midgard.
    Bonedancer = 30,

    /// Berserker class from Midgard.
    Berserker = 31,

    /// Savage class from Midgard.
    Savage = 32,

    /// Valkyrie class from Midgard.
    Valkyrie = 34,

    /// Warlock class from Midgard.
    Warlock = 59,

    /// Eldritch class from Hibernia.
    Eldritch = 40,

    /// Enchanter class from Hibernia.
    Enchanter = 41,

    /// Mentalist class from Hibernia.
    Mentalist = 42,

    /// Blademaster class from Hibernia.
    Blademaster = 43,

    /// Hero class from Hibernia.
    Hero = 44,

    /// Champion class from Hibernia.
    Champion = 45,

    /// Warden class from Hibernia.
    Warden = 46,

    /// Druid class from Hibernia.
    Druid = 47,

    /// Bard class from Hibernia.
    Bard = 48,

    /// Nightshade class from Hibernia.
    Nightshade = 49,

    /// Ranger class from Hibernia.
    Ranger = 50,

    /// Animist class from Hibernia.
    Animist = 55,

    /// Valewalker class from Hibernia.
    Valewalker = 56,

    /// Banshee class from Hibernia.
    Banshee = 39,
}

impl Class {
    /// Returns the numeric identifier of the class.
    ///
    /// # Examples
    /// ```
    /// use templess::core::domain::class::Class;
    /// let class = Class::Paladin;
    /// assert_eq!(class.id(), 1);
    /// ```
    pub fn id(&self) -> i32 {
        *self as i32
    }

    /// Returns the associated realm of the class.
    ///
    /// # Examples
    /// ```
    /// use templess::core::domain::class::Class;
    /// use templess::core::domain::realm::Realm;
    /// let class = Class::Paladin;
    /// assert_eq!(class.realm(), &Realm::Albion);
    /// ```
    pub fn realm(&self) -> &'static Realm {
        match self {
            Class::Paladin
            | Class::Armsman
            | Class::Scout
            | Class::Minstrel
            | Class::Theurgist
            | Class::Cleric
            | Class::Wizard
            | Class::Sorcerer
            | Class::Infiltrator
            | Class::Friar
            | Class::Mercenary
            | Class::Necromancer
            | Class::Cabalist
            | Class::Reaver
            | Class::Heretic => &Realm::Albion,

            Class::Thane
            | Class::Warrior
            | Class::Shadowblade
            | Class::Skald
            | Class::Hunter
            | Class::Healer
            | Class::Spiritmaster
            | Class::Shaman
            | Class::Runemaster
            | Class::Bonedancer
            | Class::Berserker
            | Class::Savage
            | Class::Valkyrie
            | Class::Warlock => &Realm::Midgard,

            Class::Eldritch
            | Class::Enchanter
            | Class::Mentalist
            | Class::Blademaster
            | Class::Hero
            | Class::Champion
            | Class::Warden
            | Class::Druid
            | Class::Bard
            | Class::Nightshade
            | Class::Ranger
            | Class::Animist
            | Class::Valewalker
            | Class::Banshee => &Realm::Hibernia,
        }
    }
}

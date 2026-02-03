//! This module defines the `Stat` enum and its associated functionality.
//!
//! Stats are an integral part of the game mechanics, representing various attributes
//! and increases to skill-lines of characters.

use strum::{Display, EnumIter, EnumString, FromRepr};

use crate::core::domain::stat_category::StatCategory;

/// Represents the various stats in the game.
///
/// Each variant corresponds to a specific stat, such as strength, dexterity, etc.
#[repr(u16)]
#[derive(
    Debug,
    Clone,
    Copy,
    FromRepr,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    Display,
    EnumIter,
    PartialOrd,
    Ord,
)]
#[strum(serialize_all = "snake_case")]
pub enum Stat {
    /// Represents the strength stat.
    ///
    /// Used for damage calculations of strength-based weapons.
    Strength = 1,

    /// Represents the dexterity stat.
    ///
    /// Used for damage calculations of dexterity-based weapons,
    /// casting speed, and block and evade chance.
    Dexterity = 2,

    /// Represents the constitution stat.
    ///
    /// Used for hit points based on class.
    Constitution = 3,

    /// Represents the quickness stat.
    ///
    /// Used for attack speed and evade chance.
    Quickness = 4,

    /// Represents the intelligence stat.
    ///
    /// Used for damage calculations and mana pool size
    /// of intelligence-based casters.
    Intelligence = 5,

    /// Represents the piety stat.
    ///
    /// Used for damage calculations and mana pool size
    /// of piety-based casters
    Piety = 6,

    /// Represents the empathy stat.
    ///
    /// Used for mana pool size
    /// of empathy-based casters.
    Empathy = 7,

    /// Represents the charisma stat.
    ///
    /// Used for damage calculations and mana pool size
    /// of charisma-based casters.
    Charisma = 8,

    /// Represents the acuity stat.
    ///
    /// Increases the amount of the primary stat for casters,
    /// such as intelligence, piety, or empathy and thus
    /// works as if it were the same primary stat.
    Acuity = 156,

    /// Represents the hitpoints stat.
    ///
    /// Used for fixed hit points regardless of class.
    Hitpoints = 10,

    /// Represents the strength stat cap increase.
    ///
    /// Used to increase the maximum value of possible strength
    /// from equipment.
    StrengthCap = 201,

    /// Represents the dexterity stat cap increase.
    ///
    /// Used to increase the maximum value of possible dexterity
    /// from equipment.
    DexterityCap = 202,

    /// Represents the constitution stat cap increase.
    ///
    /// Used to increase the maximum value of possible constitution
    /// from equipment.
    ConstitutionCap = 203,

    /// Represents the quickness stat cap increase.
    ///
    /// Used to increase the maximum value of possible quickness
    /// from equipment.
    QuicknessCap = 204,

    /// Represents the intelligence stat cap increase.
    ///
    /// Used to increase the maximum value of possible intelligence
    IntelligenceCap = 205,

    /// Represents the piety stat cap increase.
    ///
    /// Used to increase the maximum value of possible piety
    /// from equipment.
    PietyCap = 206,

    /// Represents the empathy stat cap increase.
    ///
    /// Used to increase the maximum value of possible empathy
    /// from equipment.
    EmpathyCap = 207,

    /// Represents the charisma stat cap increase.
    ///
    /// Used to increase the maximum value of possible charisma
    /// from equipment.
    CharismaCap = 208,

    /// Represents the acuity stat cap increase.
    ///
    /// Used to increase the maximum value of possible acuity
    /// from equipment.
    AcuityCap = 209,

    /// Represents the hitpoints cap increase.
    ///
    /// Used to increase the maximum value of possible hitpoints
    /// from equipment.
    HitpointsCap = 210,

    /// Represents the power pool cap increase.
    ///
    /// Used to increase the maximum value of possible power pool
    /// from equipment.
    PowerPoolCap = 211,

    /// Represents the damage resistance towards body damage.
    BodyResist = 11,

    /// Represents the damage resistance towards cold damage.
    ColdResist = 12,

    /// Represents the damage resistance towards crush damage.
    CrushResist = 13,

    /// Represents the damage resistance towards energy damage.
    EnergyResist = 14,

    /// Represents the damage resistance towards heat damage.
    HeatResist = 15,

    /// Represents the damage resistance towards matter damage.
    MatterResist = 16,

    /// Represents the damage resistance towards slash damage.
    SlashResist = 17,

    /// Represents the damage resistance towards spirit damage.
    SpiritResist = 18,

    /// Represents the damage resistance towards thrust damage.
    ThrustResist = 19,

    /// Represents the damage resistance towards essence damage.
    EssenceResist = 116,

    /// Represents the Body magic skill line.
    Body = 21,

    /// Represents the Chants magic skill line.
    Chants = 22,

    /// Represents the Death Servant magic skill line.
    DeathServant = 26,

    /// Represents the Deathsight magic skill line.
    Deathsight = 27,

    /// Represents the Earth magic skill line.
    Earth = 29,

    /// Represents the Enhancement magic skill line.
    Enhancement = 30,

    /// Represents the Fire magic skill line.
    Fire = 32,

    /// Represents the Cold magic skill line.
    Cold = 34,

    /// Represents the Instruments magic skill line.
    Instruments = 35,

    /// Represents the Matter magic skill line.
    Matter = 37,

    /// Represents the Mind magic skill line.
    Mind = 38,

    /// Represents the Pain Working magic skill line.
    PainWorking = 39,

    /// Represents the Rejuvenation magic skill line.
    Rejuvenation = 42,

    /// Represents the Smiting magic skill line.
    Smiting = 45,

    /// Represents the Soul Rending magic skill line.
    SoulRending = 46,

    /// Represents the Spirit magic skill line.
    Spirit = 47,

    /// Represents the Wind magic skill line.
    Wind = 51,

    /// Represents the Mending magic skill line.
    Mending = 57,

    /// Represents the Augmentation magic skill line.
    Augmentation = 58,

    /// Represents the Darkness magic skill line.
    Darkness = 60,

    /// Represents the Suppression magic skill line.
    Suppression = 61,

    /// Represents the Runecarving magic skill line.
    Runecarving = 62,

    /// Represents the Stormcalling magic skill line.
    Stormcalling = 63,

    /// Represents the Beastcraft magic skill line.
    Beastcraft = 64,

    /// Represents the Light magic skill line.
    Light = 65,

    /// Represents the Void magic skill line.
    Void = 66,

    /// Represents the Mana magic skill line.
    Mana = 67,

    /// Represents the Battlesongs magic skill line.
    Battlesongs = 69,

    /// Represents the Enchantments magic skill line.
    Enchantments = 70,

    /// Represents the Mentalism magic skill line.
    Mentalism = 76,

    /// Represents the Regrowth magic skill line.
    Regrowth = 77,

    /// Represents the Nurture magic skill line.
    Nurture = 78,

    /// Represents the Nature magic skill line.
    Nature = 79,

    /// Represents the Music magic skill line.
    Music = 80,

    /// Represents the Valor magic skill line.
    Valor = 84,

    /// Represents the Subterranean magic skill line.
    Subterranean = 85,

    /// Represents the Bone Army magic skill line.
    BoneArmy = 86,

    /// Represents the Verdant magic skill line.
    Verdant = 87,

    /// Represents the Creeping magic skill line.
    Creeping = 88,

    /// Represents the Arboreal magic skill line.
    Arboreal = 89,

    /// Represents the Pacification magic skill line.
    Pacification = 94,

    /// Represents the Summoning magic skill line.
    Summoning = 98,

    /// Represents all magic skill lines.
    AllMagicSkills = 163,

    /// Represents the Two Handed weapon skill line.
    TwoHanded = 20,

    /// Represents the Crushing weapon skill line.
    Crushing = 25,

    /// Represents the Flexible Weapon skill line.
    FlexibleWeapon = 33,

    /// Represents the Polearm weapon skill line.
    Polearm = 41,

    /// Represents the Slashing weapon skill line.
    Slashing = 44,

    /// Represents the Staff weapon skill line.
    Staff = 48,

    /// Represents the Thrusting weapon skill line.
    Thrusting = 50,

    /// Represents the Sword weapon skill line.
    Sword = 52,

    /// Represents the Hammer weapon skill line.
    Hammer = 53,

    /// Represents the Axe weapon skill line.
    Axe = 54,

    /// Represents the Spear weapon skill line.
    Spear = 56,

    /// Represents the Blade weapon skill line.
    Blade = 72,

    /// Represents the Blunt weapon skill line.
    Blunt = 73,

    /// Represents the Piercing weapon skill line.
    Piercing = 74,

    /// Represents the Large Weapon skill line.
    LargeWeapon = 75,

    /// Represents the Celtic Spear weapon skill line.
    CelticSpear = 82,

    /// Represents the Scythe weapon skill line.
    Scythe = 90,

    /// Represents the Hand To Hand weapon skill line.
    HandToHand = 92,

    /// Represents all melee skill lines.
    AllMeleeSkills = 164,

    /// Represents all archery skill lines.
    AllArcherySkills = 168,

    /// Represents the Short Bow weapon skill line.
    ShortBow = 93,

    /// Represents the Recurve Bow weapon skill line.
    RecurveBow = 83,

    /// Represents the Composite Bow weapon skill line.
    Composite = 68,

    /// Represents the Long Bow weapon skill line.
    LongBow = 36,

    /// Represents the Thrown Weapon skill line.
    ThrownWeapon = 91,

    /// Represents the Crossbow weapon skill line.
    Crossbow = 24,

    /// Represents all dual wielding skill lines.
    AllDualWieldingSkills = 167,

    /// Represents the Celtic Dual weapon skill line.
    CelticDual = 81,

    /// Represents the Left Axe weapon skill line.
    LeftAxe = 55,

    /// Represents the Dual Wield weapon skill line.
    DualWield = 28,

    /// Represents the Critical Strike skill line.
    CriticalStrike = 23,

    /// Represents the Shield skill line.
    Shield = 43,

    /// Represents the Parry skill line.
    Parry = 40,

    /// Represents the Envenom skill line.
    Envenom = 31,

    /// Represents the Stealth skill line.
    Stealth = 49,

    /// Represents the Savagery skill line.
    Savagery = 95,

    /// Represents the Nightshade skill line.
    Nightshade = 96,

    /// Represents the Pathfinding skill line.
    Pathfinding = 97,

    /// Represents the Armor Factor stat.
    ArmorFactor = 148,

    /// Represents the Spell Range stat.
    SpellRange = 153,

    /// Represents the Melee Speed stat.
    MeleeSpeed = 155,

    /// Represents the Melee Damage stat.
    MeleeDamage = 173,

    /// Represents the Ranged Damage stat.
    RangedDamage = 174,

    /// Represents the Archery Speed stat.
    ArcherySpeed = 188,

    /// Represents the Buff Effectiveness stat.
    BuffEffectiveness = 190,

    /// Represents the Casting Speed stat.
    CastingSpeed = 191,

    /// Represents the Debuff Effectiveness stat.
    DebuffEffectiveness = 193,

    /// Represents the Healing Effectiveness stat.
    HealingEffectiveness = 195,

    /// Represents the Power Pool ToA stat.
    PowerPoolToa = 196,

    /// Represents the Resist Pierce stat.
    ResistPierce = 197,

    /// Represents the Spell Damage stat.
    SpellDamage = 198,

    /// Represents the Spell Duration stat.
    SpellDuration = 199,

    /// Represents the Style Damage stat.
    StyleDamage = 200,

    /// Represents the Arcane Syphon stat.
    ArcaneSyphon = 254,

    /// Represents the Fatigue stat.
    Fatigue = 194,

    /// Represents all focus level stats.
    AllFocusLevels = 165,

    /// Represents an unknown ranger-related stat.
    UnknownRangerStuff = 115,

    /// Represents an unknown stat.
    UnknownStuffAgain = 99,

    /// Represents another unknown stat.
    UnknownStuffAgain2 = 101,
}
struct StatInfo {
    pub utility: f32,
    pub cap: u16,
    pub category: StatCategory,
}

impl Stat {
    const fn info(self) -> StatInfo {
        use Stat::*;
        match self {
            Strength | Dexterity | Constitution | Quickness => StatInfo {
                utility: 0.66,
                cap: 75,
                category: StatCategory::PhysicalStats,
            },
            Intelligence | Piety | Empathy | Charisma | Acuity => StatInfo {
                utility: 0.66,
                cap: 75,
                category: StatCategory::AcuityStats,
            },
            Hitpoints => StatInfo {
                utility: 0.25,
                cap: 200,
                category: StatCategory::PhysicalStats,
            },

            StrengthCap | DexterityCap | ConstitutionCap | QuicknessCap => StatInfo {
                utility: 2.0,
                cap: 26,
                category: StatCategory::PhysicalStatCaps,
            },
            IntelligenceCap | PietyCap | EmpathyCap | CharismaCap | AcuityCap | PowerPoolCap => StatInfo {
                utility: 2.0,
                cap: 26,
                category: StatCategory::AcuityStatCaps,
            },
            HitpointsCap => StatInfo {
                utility: 2.0,
                cap: 200,
                category: StatCategory::PhysicalStatCaps,
            },

            BodyResist | ColdResist | CrushResist | EnergyResist | HeatResist | MatterResist
            | SlashResist | SpiritResist | ThrustResist | EssenceResist => StatInfo {
                utility: 2.0,
                cap: 26,
                category: StatCategory::Resists,
            },

            Body | Chants | DeathServant | Deathsight | Earth | Enhancement | Fire | Cold
            | Instruments | Matter | Mind | PainWorking | Rejuvenation | Smiting | SoulRending
            | Spirit | Wind | Mending | Augmentation | Darkness | Suppression | Runecarving
            | Stormcalling | Beastcraft | Light | Void | Mana | Battlesongs | Enchantments
            | Mentalism | Regrowth | Nurture | Nature | Music | Valor | Subterranean | BoneArmy
            | Verdant | Creeping | Arboreal | Pacification | Summoning | AllMagicSkills => {
                StatInfo {
                    utility: 5.0,
                    cap: 11,
                    category: StatCategory::MagicSkills,
                }
            }
            TwoHanded | Crushing | FlexibleWeapon | Polearm | Slashing | Staff | Thrusting
            | Sword | Hammer | Axe | Spear | Blade | Blunt | Piercing | LargeWeapon
            | CelticSpear | Scythe | HandToHand | AllMeleeSkills => StatInfo {
                utility: 5.0,
                cap: 11,
                category: StatCategory::MeleeSkills,
            },
            AllArcherySkills | ShortBow | RecurveBow | Composite | LongBow | ThrownWeapon
            | Crossbow => StatInfo {
                utility: 5.0,
                cap: 11,
                category: StatCategory::ArcherySkills,
            },
            AllDualWieldingSkills | CelticDual | LeftAxe | DualWield => StatInfo {
                utility: 5.0,
                cap: 11,
                category: StatCategory::DualWieldingSkills,
            },
            CriticalStrike | Shield | Parry | Envenom | Stealth | Savagery | Nightshade
            | Pathfinding => StatInfo {
                utility: 5.0,
                cap: 11,
                category: StatCategory::OtherSkills,
            },

            ArmorFactor => StatInfo {
                utility: 1.0,
                cap: 50,
                category: StatCategory::OtherStats,
            },

            SpellRange | BuffEffectiveness | DebuffEffectiveness | HealingEffectiveness
            | PowerPoolToa | SpellDuration => StatInfo {
                utility: 2.0,
                cap: 25,
                category: StatCategory::ToaBonuses,
            },

            MeleeSpeed | MeleeDamage | RangedDamage | ArcherySpeed | CastingSpeed
            | ResistPierce | SpellDamage | StyleDamage | ArcaneSyphon => StatInfo {
                utility: 2.0,
                cap: 10,
                category: StatCategory::ToaBonuses,
            },

            Fatigue => StatInfo {
                utility: 0.0,
                cap: 100,
                category: StatCategory::OtherStats,
            },
            AllFocusLevels => StatInfo {
                utility: 0.0,
                cap: 50,
                category: StatCategory::OtherStats,
            },
            UnknownRangerStuff | UnknownStuffAgain | UnknownStuffAgain2 => StatInfo {
                utility: 0.0,
                cap: 100,
                category: StatCategory::OtherStats,
            },
        }
    }

    /// Returns the name of the stat.
    pub fn name(&self) -> String {
        self.to_string()
    }

    /// Returns the ID of the stat as a `u16`.
    pub fn id(&self) -> u16 {
        *self as u16
    }

    /// Returns the ID of the stat as an `f32`.
    pub fn utility_per_point(&self) -> f32 {
        self.info().utility
    }

    /// Returns the cap value of the stat.
    pub fn cap(&self) -> u16 {
        self.info().cap
    }

    /// Returns the category associated with this stat.
    pub fn category(&self) -> StatCategory {
        self.info().category
    }

    pub fn base_stat(&self) -> Option<Stat> {
        use Stat::*;
        match self {
            StrengthCap => Some(Strength),
            DexterityCap => Some(Dexterity),
            ConstitutionCap => Some(Constitution),
            QuicknessCap => Some(Quickness),
            IntelligenceCap => Some(Intelligence),
            PietyCap => Some(Piety),
            EmpathyCap => Some(Empathy),
            CharismaCap => Some(Charisma),
            AcuityCap => Some(Acuity),
            HitpointsCap => Some(Hitpoints),
            PowerPoolCap => Some(PowerPoolToa),
            _ => None,
        }
    }

    pub fn cap_stat(&self) -> Option<Self> {
        match self {
            Stat::Strength => Some(Stat::StrengthCap),
            Stat::Dexterity => Some(Stat::DexterityCap),
            Stat::Constitution => Some(Stat::ConstitutionCap),
            Stat::Quickness => Some(Stat::QuicknessCap),
            Stat::Intelligence => Some(Stat::IntelligenceCap),
            Stat::Piety => Some(Stat::PietyCap),
            Stat::Empathy => Some(Stat::EmpathyCap),
            Stat::Charisma => Some(Stat::CharismaCap),
            Stat::Acuity => Some(Stat::AcuityCap),
            Stat::Hitpoints => Some(Stat::HitpointsCap),
            Stat::PowerPoolToa => Some(Stat::PowerPoolCap),
            _ => None,
        }
    }
}

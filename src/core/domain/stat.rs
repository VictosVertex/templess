//! This module defines the `Stat` enum and its associated functionality.
//!
//! Stats are an integral part of the game mechanics, representing various attributes
//! and increases to skill-lines of characters.

use strum::FromRepr;

/// Represents the various stats in the game.
///
/// Each variant corresponds to a specific stat, such as strength, dexterity, etc.
#[repr(u16)]
#[derive(Debug, Clone, Copy, FromRepr, PartialEq)]
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

type StatInfo = (&'static str, f32, i32);

/// Maps stat IDs to their names, utility per point, and cap values.
///
/// # Parameters
/// - `id`: The ID of the stat.
///
/// # Returns
/// A tuple containing the name of the stat, its utility per point, and its cap value.
fn stat_info_map(id: u16) -> StatInfo {
    match id {
        1u16 => ("strength", 0.67, 75),
        2u16 => ("dexterity", 0.67, 75),
        3u16 => ("constitution", 0.67, 75),
        4u16 => ("quickness", 0.67, 75),
        5u16 => ("intelligence", 0.67, 75),
        6u16 => ("piety", 0.67, 75),
        7u16 => ("empathy", 0.67, 75),
        8u16 => ("charisma", 0.67, 75),
        156u16 => ("acuity", 0.67, 75),
        10u16 => ("hitpoints", 0.25, 200),
        201u16 => ("strength_cap", 2.0, 26),
        202u16 => ("dexterity_cap", 2.0, 26),
        203u16 => ("constitution_cap", 2.0, 26),
        204u16 => ("quickness_cap", 2.0, 26),
        205u16 => ("intelligence_cap", 2.0, 26),
        206u16 => ("piety_cap", 2.0, 26),
        207u16 => ("empathy_cap", 2.0, 26),
        208u16 => ("charisma_cap", 2.0, 26),
        209u16 => ("acuity_cap", 2.0, 26),
        210u16 => ("hitpoints_cap", 2.0, 200),
        211u16 => ("power_pool_cap", 2.0, 26),
        11u16 => ("body_resist", 2.0, 26),
        12u16 => ("cold_resist", 2.0, 26),
        13u16 => ("crush_resist", 2.0, 26),
        14u16 => ("energy_resist", 2.0, 26),
        15u16 => ("heat_resist", 2.0, 26),
        16u16 => ("matter_resist", 2.0, 26),
        17u16 => ("slash_resist", 2.0, 26),
        18u16 => ("spirit_resist", 2.0, 26),
        19u16 => ("thrust_resist", 2.0, 26),
        116u16 => ("essence_resist", 2.0, 26),
        21u16 => ("body", 5.0, 11),
        22u16 => ("chants", 5.0, 11),
        26u16 => ("death_servant", 5.0, 11),
        27u16 => ("deathsight", 5.0, 11),
        29u16 => ("earth", 5.0, 11),
        30u16 => ("enhancement", 5.0, 11),
        32u16 => ("fire", 5.0, 11),
        34u16 => ("cold", 5.0, 11),
        35u16 => ("instruments", 5.0, 11),
        37u16 => ("matter", 5.0, 11),
        38u16 => ("mind", 5.0, 11),
        39u16 => ("pain_working", 5.0, 11),
        42u16 => ("rejuvenation", 5.0, 11),
        45u16 => ("smiting", 5.0, 11),
        46u16 => ("soul_rending", 5.0, 11),
        47u16 => ("spirit", 5.0, 11),
        51u16 => ("wind", 5.0, 11),
        57u16 => ("mending", 5.0, 11),
        58u16 => ("augmentation", 5.0, 11),
        60u16 => ("darkness", 5.0, 11),
        61u16 => ("suppression", 5.0, 11),
        62u16 => ("runecarving", 5.0, 11),
        63u16 => ("stormcalling", 5.0, 11),
        64u16 => ("beastcraft", 5.0, 11),
        65u16 => ("light", 5.0, 11),
        66u16 => ("void", 5.0, 11),
        67u16 => ("mana", 5.0, 11),
        69u16 => ("battlesongs", 5.0, 11),
        70u16 => ("enchantments", 5.0, 11),
        76u16 => ("mentalism", 5.0, 11),
        77u16 => ("regrowth", 5.0, 11),
        78u16 => ("nurture", 5.0, 11),
        79u16 => ("nature", 5.0, 11),
        80u16 => ("music", 5.0, 11),
        84u16 => ("valor", 5.0, 11),
        85u16 => ("subterranean", 5.0, 11),
        86u16 => ("bone_army", 5.0, 11),
        87u16 => ("verdant", 5.0, 11),
        88u16 => ("creeping", 5.0, 11),
        89u16 => ("arboreal", 5.0, 11),
        94u16 => ("pacification", 5.0, 11),
        98u16 => ("summoning", 5.0, 11),
        163u16 => ("all_magic_skills", 5.0, 11),
        20u16 => ("two_handed", 5.0, 11),
        25u16 => ("crushing", 5.0, 11),
        33u16 => ("flexible_weapon", 5.0, 11),
        41u16 => ("polearm", 5.0, 11),
        44u16 => ("slashing", 5.0, 11),
        48u16 => ("staff", 5.0, 11),
        50u16 => ("thrusting", 5.0, 11),
        52u16 => ("sword", 5.0, 11),
        53u16 => ("hammer", 5.0, 11),
        54u16 => ("axe", 5.0, 11),
        56u16 => ("spear", 5.0, 11),
        72u16 => ("blade", 5.0, 11),
        73u16 => ("blunt", 5.0, 11),
        74u16 => ("piercing", 5.0, 11),
        75u16 => ("large_weapon", 5.0, 11),
        82u16 => ("celtic_spear", 5.0, 11),
        90u16 => ("scythe", 5.0, 11),
        92u16 => ("hand_to_hand", 5.0, 11),
        164u16 => ("all_melee_skills", 5.0, 11),
        168u16 => ("all_archery_skills", 5.0, 11),
        93u16 => ("short_bow", 5.0, 11),
        83u16 => ("recurve_bow", 5.0, 11),
        68u16 => ("composite", 5.0, 11),
        36u16 => ("long_bow", 5.0, 11),
        91u16 => ("thrown_weapon", 5.0, 11),
        24u16 => ("crossbow", 5.0, 11),
        167u16 => ("all_dual_wielding_skills", 5.0, 11),
        81u16 => ("celtic_dual", 5.0, 11),
        55u16 => ("left_axe", 5.0, 11),
        28u16 => ("dual_wield", 5.0, 11),
        23u16 => ("critical_strike", 5.0, 11),
        43u16 => ("shield", 5.0, 11),
        40u16 => ("parry", 5.0, 11),
        31u16 => ("envenom", 5.0, 11),
        49u16 => ("stealth", 5.0, 11),
        95u16 => ("savagery", 5.0, 11),
        96u16 => ("nightshade", 5.0, 11),
        97u16 => ("pathfinding", 5.0, 11),
        148u16 => ("armor_factor", 1.0, 50),
        153u16 => ("spell_range", 2.0, 25),
        155u16 => ("melee_speed", 2.0, 10),
        173u16 => ("melee_damage", 2.0, 10),
        174u16 => ("ranged_damage", 2.0, 10),
        188u16 => ("archery_speed", 2.0, 10),
        190u16 => ("buff_effectiveness", 2.0, 25),
        191u16 => ("casting_speed", 2.0, 10),
        193u16 => ("debuff_effectiveness", 2.0, 25),
        195u16 => ("healing_effectiveness", 2.0, 25),
        196u16 => ("power_pool_toa", 2.0, 25),
        197u16 => ("resist_pierce", 2.0, 10),
        198u16 => ("spell_damage", 2.0, 10),
        199u16 => ("spell_duration", 2.0, 25),
        200u16 => ("style_damage", 2.0, 10),
        254u16 => ("arcane_syphon", 2.0, 10),
        194u16 => ("fatigue", 0.0, 100),
        165u16 => ("all_focus_levels", 0.0, 50),
        115u16 => ("unknown_ranger_stuff", 0.0, 100),
        99u16 => ("unknown_stuff_again", 0.0, 100),
        101u16 => ("unknown_stuff_again2", 0.0, 100),
        _ => ("unknown", 0.0, 0),
    }
}

impl Stat {
    /// Returns the name of the stat.
    pub fn name(&self) -> &'static str {
        stat_info_map(self.id()).0
    }

    /// Returns the ID of the stat as a `u16`.
    pub fn id(&self) -> u16 {
        *self as u16
    }

    /// Returns the ID of the stat as an `f32`.
    pub fn utility_per_point(&self) -> f32 {
        stat_info_map(self.id()).1
    }

    /// Returns the cap value of the stat.
    pub fn cap(&self) -> u16 {
        stat_info_map(self.id()).2 as u16
    }

    /// Returns the category IDs associated with this stat.
    pub fn category_ids(&self) -> &'static [u8] {
        &[1, 2, 3]
    }
}

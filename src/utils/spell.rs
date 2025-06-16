use super::{ActionType, Advantage, Attribute, DamageType, Die};

// TODO: Add school of magic!
//
/// Represents a spell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spell {
    /// The minimum spell slot level required to cast the spell.
    pub level: u8,

    /// The time required to cast the spell.
    pub casting_time: CastingTime,

    /// The max range of the spell.
    pub range: u8,

    /// The required components for the spell.
    pub components: Vec<Components>,

    /// The duration of the spell.
    pub duration: Duration,

    /// The effects the spell has.
    pub effects: Vec<SpellEffect>,

    /// Extra effects of the spell if casted at a higher level.
    pub upcast: Option<Upcast>,

    /// Extra effects of the spell at higher level character levels.
    pub higher_levels: Option<Vec<HigherLevelEffect>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CastingTime {
    Action(ActionType),
    Time(std::time::Duration),
}

/// Represents spell components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Components {
    Verbal,
    Somatic,
    Material(String),
}

/// Represents spell duration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Duration {
    Instantaneous,
    Round(u8),
    StartOfNextTurn,
    EndOfNextTurn,
    Time(std::time::Duration),
    Concentration(std::time::Duration),
}

/// Represents spell effects when upcast.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upcast {
    min_slot_lvl: u8,
    effects: Vec<SpellEffect>,
}

/// Represents effects a spell gains at higher character levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HigherLevelEffect {
    required_level: u8,
    effects: Vec<SpellEffect>,
}

/// Represents effects spells can have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellEffect {
    ExtraDamage(Damage),
    TemporaryHP(u16),
    SavingThrow {
        attribute: Attribute,
        advantage: Option<Advantage>,
        effects: Vec<SpellEffect>,
    },
    IncreaseAC(u8),
    AttackRoll(AttackRoll),
    Other(String),
}

/// Represents some damage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Damage {
    damage_type: Vec<DamageType>,
    damage_amount: Vec<DamageAmount>,
}

/// Represents a damage amount.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageAmount {
    Flat(u8),
    Roll { dice: Vec<Die>, num_dice: usize },
}

/// Represents an attack roll.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackRoll {
    Melee,
    Range,
}

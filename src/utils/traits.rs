use crate::utils::{ActionType, Attribute, Choice, Proficiency, Spell};

/// Represents a trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trait {
    /// The name of the trait.
    pub name: String,

    /// The trait's description.
    pub summary: String,

    /// The effects the trait provides.
    pub effects: Vec<TraitEffect>,

    /// The required level to gain access to the trait.
    pub required_level: Option<u8>,

    /// Tags the trait belongs under.
    pub tags: Vec<String>,
}

/// Represents effects traits can have.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraitEffect {
    // NOTE: https://www.reddit.com/r/dndnext/comments/ourpif/dnd5e_light_and_vision_quick_reference_chart/?rdt=62795
    //
    /// A trait that effects the vision of the character.
    Vision(Vision),

    /// A trait that effects the saving throws of the character.
    SavingThrows {
        advantage: Advantage,
        kind: SavingThrowType,
    },

    /// A trait that effects the resistances and vulnerabilities of the character.
    Resistances(Resistance),

    /// A trait that effects the proficiencies of the character.
    Proficiencies(Choice<Proficiency>),

    /// A trait that gives access to a spell.
    Spell(Spell),

    /// A trait that provides some type of action.
    Action {
        kind: ActionType,
        effects: Vec<Self>,
    },

    /// A trait that provides increased hit points.
    HpIncrease(HpIncrease),

    /// A trait that causes no speed reduction.
    NoSpeedReduction,
}

impl std::fmt::Display for TraitEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraitEffect::Vision(_) => f.write_str("Vision"),
            TraitEffect::SavingThrows { .. } => f.write_str("Saving Throws"),
            TraitEffect::Resistances(_) => f.write_str("Resistances"),
            TraitEffect::Proficiencies(_) => f.write_str("Proficiencies"),
            TraitEffect::Spell(_) => f.write_str("Spell"),
            TraitEffect::Action { .. } => f.write_str("Action"),
            TraitEffect::HpIncrease(_) => f.write_str("Health Increase"),
            TraitEffect::NoSpeedReduction => f.write_str("No Speed Reduction"),
        }
    }
}

/// Represents the types of visions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vision {
    Normal(u16),
    Darkvision(u16),
    Truesight(u16),
    DevilsSight(u16),
}

impl Vision {
    pub fn value(&self) -> u16 {
        match self {
            Vision::Normal(amt) => *amt,
            Vision::Darkvision(amt) => *amt,
            Vision::Truesight(amt) => *amt,
            Vision::DevilsSight(amt) => *amt,
        }
    }

    pub fn set_value(&mut self, value: u16) {
        match self {
            Vision::Normal(amt) => *amt = value,
            Vision::Darkvision(amt) => *amt = value,
            Vision::Truesight(amt) => *amt = value,
            Vision::DevilsSight(amt) => *amt = value,
        }
    }
}

impl std::fmt::Display for Vision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vision::Normal(_) => f.write_str("Normal"),
            Vision::Darkvision(_) => f.write_str("Darkvision"),
            Vision::Truesight(_) => f.write_str("Truesight"),
            Vision::DevilsSight(_) => f.write_str("Devil's Sight"),
        }
    }
}

/// Represents a type of saving throw.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SavingThrowType {
    Attribute(Attribute),
    Damage(DamageType),
}

impl std::fmt::Display for SavingThrowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SavingThrowType::Attribute(_) => f.write_str("Attribute"),
            SavingThrowType::Damage(_) => f.write_str("Damage Type"),
        }
    }
}

/// Represents advantage or disadvantage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Advantage {
    Advantage,
    Disadvantage,
}

impl std::fmt::Display for Advantage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Advantage::Advantage => f.write_str("Advantage"),
            Advantage::Disadvantage => f.write_str("Disadvantage"),
        }
    }
}

/// Represents the various types of damage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

/// Represents the either a resistance or vulnerability to a damage type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resistance {
    Resistance(DamageType),
    Vulnerability(DamageType),
}

/// Represents an increase in hit points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HpIncrease {
    /// Increase the max hit points.
    Max(u16),

    /// Increase the max hit points every level.
    PerLevel(u16),
}

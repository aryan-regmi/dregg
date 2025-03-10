use super::{Attribute, Choice, Proficiency, Spell};

/// Represents a trait.
#[derive(Debug)]
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
#[derive(Debug)]
pub enum TraitEffect {
    // NOTE: https://www.reddit.com/r/dndnext/comments/ourpif/dnd5e_light_and_vision_quick_reference_chart/?rdt=62795
    //
    /// A trait that effects the vision of the character.
    Vision(Vision),

    /// A trait that effects the saving throws of the character.
    SavingThrows {
        advantage: Advantage,
        attribute: Attribute,
    },

    /// A trait that effects the resistances and vulnerabilities of the character.
    Resistances(Resistance),

    /// A trait that effects the proficiencies of the character.
    Proficiencies(Choice<Proficiency>),

    /// A trait that gives access to a spell.
    Spell(Choice<Spell>),
}

/// Represents the types of visions.
#[derive(Debug)]
pub enum Vision {
    Normal(u16),
    Darkvision(u16),
    Truesight(u16),
    DevilsSight(u16),
}

/// Represents advantage or disadvantage.
#[derive(Debug)]
pub enum Advantage {
    Advantage,
    Disadvantage,
}

/// Represents the various types of damage.
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Resistance {
    Resistance(DamageType),
    Vulnerability(DamageType),
}

use super::{Attribute, Choice, Proficiency};

/// Represents a trait.
#[derive(Debug)]
pub struct Trait {
    /// The name of the trait.
    pub name: String,

    /// The trait's description.
    pub summary: String,

    /// The effects the trait provides.
    pub effects: Vec<TraitEffect>,

    /// Tags the trait belongs under.
    pub tags: Vec<String>,
}

// TODO: Update with actual inner types
//
/// Represents trait effects can have.
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

// TODO: Fill will rest!
//
/// Represents the various types of damage.
#[derive(Debug)]
pub enum DamageType {
    Piercing,
    Poison,
}

/// Represents the either a resistance or vulnerability to a damage type.
#[derive(Debug)]
pub enum Resistance {
    Resistance(DamageType),
    Vulnerability(DamageType),
}

#![allow(dead_code)]

pub mod background;
pub mod character;
pub mod class;
pub mod equipment;
pub mod race;

/// Represents a range of possible values.
#[derive(Debug)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

/// Represents various choices a character can make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice<T> {
    /// A list of choices out of which only one can be selected.
    OnlyOne(Vec<T>),

    /// A list of choices from which all are selected.
    All(Vec<T>),

    /// A single choice.
    Single(T),
}

/// Represents ability scores.
#[derive(Debug)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

impl Default for AbilityScores {
    fn default() -> Self {
        Self {
            strength: 8,
            dexterity: 8,
            constitution: 8,
            intelligence: 8,
            wisdom: 8,
            charisma: 8,
        }
    }
}

/// Represents a summary/description.
#[derive(Debug)]
pub struct Summary {
    pub main: String,
    pub subsections: Vec<(String, String)>,
}

/// Represents an attribute.
#[derive(Debug)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

/// Represents a height in feet and inches.
#[derive(Debug)]
pub struct Height {
    pub feet: f32,
    pub inches: f32,
}

/// Represents a height in pounds.
#[derive(Debug)]
pub struct Weight(f32);

/// Represents a speed of a character.
#[derive(Debug)]
pub enum Speed {
    Walking(u16),
    Flying(u16),
    Swimming(u16),
    Climbing(u16),
}

/// Represents a language a character knows.
#[derive(Debug)]
pub struct Language {
    pub name: String,
    pub levels: Vec<LanguageLevel>,
}

/// Represents the various levels of proficiency in a language.
#[derive(Debug)]
pub enum LanguageLevel {
    Speak,
    Read,
    Write,
    Understand,
}

// TODO: Update with actual inner types
//
/// Represents a proficiency.
#[derive(Debug)]
pub struct Proficiency {
    /// The proficiency level (Proficient or Expertise).
    pub level: ProficiencyLevel,

    /// Determines what the proficiency applies to.
    pub kind: ProficiencyType,

    /// Extra context that is required for proficiencies with conditional activations.
    pub context: Option<String>,
}

/// Represents a type of proficiency.
#[derive(Debug)]
pub enum ProficiencyType {
    Armor,
    Weapons,
    Tools,
    SavingThrows,
    Skills,
}

/// Represents a level of proficiency.
#[derive(Debug)]
pub enum ProficiencyLevel {
    Proficient,
    Expertise,
}

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

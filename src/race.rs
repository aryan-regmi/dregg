use std::fmt::Debug;

use crate::common::{size::Size, Attribute, Language, Speed, Summary, Trait};

/// Represents a race.
#[derive(Debug)]
pub struct Race {
    /// Gets the name of the race.
    pub name: String,

    /// Gets the plural form of the race's name.
    pub name_plural: String,

    /// Gets the description of the race.
    pub summary: Summary,

    /// Gets the ability score increases provided by the race.
    pub asi: Option<Vec<Attribute>>,

    /// Gets the age info of the race.
    pub age: Option<Age>,

    /// Gets the size of the race.
    pub size: Size,

    /// Gets the speed of the race.
    pub speed: Speed,

    /// Gets the languages provided by the race.
    pub languages: Option<Vec<Language>>,

    pub traits: Vec<Trait>,
}

/// Represents the age info for a race.
#[derive(Debug)]
pub struct Age {
    /// The age at which a character of this race is considered an adult.
    pub adult: Option<u16>,

    /// The average lifespan of this race.
    pub lifespan: Option<u16>,
}
